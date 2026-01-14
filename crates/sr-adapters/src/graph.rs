//! Dependency Graph Projection per SR-SPEC §1.8
//!
//! D-12: Implements graph projection builder and staleness traversal.
//!
//! Key capabilities:
//! - Derive nodes/edges from typed refs in events
//! - Transitive dependency queries (get_dependencies)
//! - Impact analysis queries (get_dependents)
//! - Staleness propagation and resolution

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use sr_domain::{ActorKind, EventEnvelope, TypedRef};
use tracing::{debug, error, info, instrument, warn};
use ulid::Ulid;

/// Graph projection error types
#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("Node not found: {node_id}")]
    NodeNotFound { node_id: String },

    #[error("Invalid edge type: {edge_type}")]
    InvalidEdgeType { edge_type: String },
}

impl From<sqlx::Error> for GraphError {
    fn from(e: sqlx::Error) -> Self {
        GraphError::DatabaseError {
            message: e.to_string(),
        }
    }
}

/// Valid edge types per SR-SPEC Appendix B
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    About,
    DependsOn,
    Produces,
    Verifies,
    ApprovedBy,
    Acknowledges,
    Supersedes,
    Releases,
    SupportedBy,
    GovernedBy,
    InScopeOf,
    Affects,
    Stale,
    RootCause,
    RelatesTo,
}

impl EdgeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EdgeType::About => "about",
            EdgeType::DependsOn => "depends_on",
            EdgeType::Produces => "produces",
            EdgeType::Verifies => "verifies",
            EdgeType::ApprovedBy => "approved_by",
            EdgeType::Acknowledges => "acknowledges",
            EdgeType::Supersedes => "supersedes",
            EdgeType::Releases => "releases",
            EdgeType::SupportedBy => "supported_by",
            EdgeType::GovernedBy => "governed_by",
            EdgeType::InScopeOf => "in_scope_of",
            EdgeType::Affects => "affects",
            EdgeType::Stale => "stale",
            EdgeType::RootCause => "root_cause",
            EdgeType::RelatesTo => "relates_to",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "about" => Some(EdgeType::About),
            "depends_on" => Some(EdgeType::DependsOn),
            "produces" => Some(EdgeType::Produces),
            "verifies" => Some(EdgeType::Verifies),
            "approved_by" => Some(EdgeType::ApprovedBy),
            "acknowledges" => Some(EdgeType::Acknowledges),
            "supersedes" => Some(EdgeType::Supersedes),
            "releases" => Some(EdgeType::Releases),
            "supported_by" => Some(EdgeType::SupportedBy),
            "governed_by" => Some(EdgeType::GovernedBy),
            "in_scope_of" => Some(EdgeType::InScopeOf),
            "affects" => Some(EdgeType::Affects),
            "stale" => Some(EdgeType::Stale),
            "root_cause" => Some(EdgeType::RootCause),
            "relates_to" => Some(EdgeType::RelatesTo),
            _ => None,
        }
    }
}

/// Staleness reason codes per SR-SPEC §1.13
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StalenessReason {
    GovernedArtifactChanged,
    OracleSuiteRebased,
    ExceptionActivated,
    DependencyStale,
    ManualMark,
}

impl StalenessReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            StalenessReason::GovernedArtifactChanged => "GOVERNED_ARTIFACT_CHANGED",
            StalenessReason::OracleSuiteRebased => "ORACLE_SUITE_REBASED",
            StalenessReason::ExceptionActivated => "EXCEPTION_ACTIVATED",
            StalenessReason::DependencyStale => "DEPENDENCY_STALE",
            StalenessReason::ManualMark => "MANUAL_MARK",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "GOVERNED_ARTIFACT_CHANGED" => Some(StalenessReason::GovernedArtifactChanged),
            "ORACLE_SUITE_REBASED" => Some(StalenessReason::OracleSuiteRebased),
            "EXCEPTION_ACTIVATED" => Some(StalenessReason::ExceptionActivated),
            "DEPENDENCY_STALE" => Some(StalenessReason::DependencyStale),
            "MANUAL_MARK" => Some(StalenessReason::ManualMark),
            _ => None,
        }
    }
}

/// Graph node
#[derive(Debug, Clone)]
pub struct GraphNode {
    pub node_id: String,
    pub node_type: String,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub meta: serde_json::Value,
}

/// Graph edge
#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub edge_id: i64,
    pub src_id: String,
    pub dst_id: String,
    pub edge_type: String,
    pub created_at: DateTime<Utc>,
    pub meta: serde_json::Value,
}

/// Staleness marker
#[derive(Debug, Clone)]
pub struct StalenessMarker {
    pub stale_id: String,
    pub root_kind: String,
    pub root_id: String,
    pub dependent_kind: String,
    pub dependent_id: String,
    pub reason_code: String,
    pub reason_detail: Option<String>,
    pub marked_at: DateTime<Utc>,
    pub marked_by_kind: String,
    pub marked_by_id: String,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_event_id: Option<String>,
}

/// Dependency edge result from traversal
#[derive(Debug, Clone)]
pub struct DependencyEdge {
    pub src_id: String,
    pub dst_id: String,
    pub edge_type: String,
    pub depth: i32,
}

/// Graph projection builder
///
/// Per SR-SPEC §1.8: Maintains the dependency graph projection
/// and supports staleness traversal per §1.13.
pub struct GraphProjection {
    pool: PgPool,
}

impl GraphProjection {
    /// Create a new graph projection
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get a reference to the underlying pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    // ========================================================================
    // Node Operations
    // ========================================================================

    /// Ensure a node exists in the graph
    #[instrument(skip(self))]
    pub async fn ensure_node(
        &self,
        node_id: &str,
        node_type: &str,
        label: Option<&str>,
        meta: Option<&serde_json::Value>,
    ) -> Result<(), GraphError> {
        let meta_value = meta.cloned().unwrap_or(serde_json::json!({}));

        sqlx::query(
            r#"
            INSERT INTO graph.nodes (node_id, node_type, label, meta)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (node_id) DO UPDATE
            SET node_type = $2, label = COALESCE($3, graph.nodes.label), meta = $4
            "#,
        )
        .bind(node_id)
        .bind(node_type)
        .bind(label)
        .bind(meta_value)
        .execute(&self.pool)
        .await?;

        debug!(node_id = node_id, node_type = node_type, "Node ensured");
        Ok(())
    }

    /// Get a node by ID
    pub async fn get_node(&self, node_id: &str) -> Result<Option<GraphNode>, GraphError> {
        let result = sqlx::query_as::<
            _,
            (
                String,
                String,
                Option<String>,
                DateTime<Utc>,
                serde_json::Value,
            ),
        >(
            r#"
            SELECT node_id, node_type, label, created_at, meta
            FROM graph.nodes WHERE node_id = $1
            "#,
        )
        .bind(node_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(
            result.map(|(node_id, node_type, label, created_at, meta)| GraphNode {
                node_id,
                node_type,
                label,
                created_at,
                meta,
            }),
        )
    }

    // ========================================================================
    // Edge Operations
    // ========================================================================

    /// Add an edge between two nodes
    #[instrument(skip(self))]
    pub async fn add_edge(
        &self,
        src_id: &str,
        dst_id: &str,
        edge_type: EdgeType,
        meta: Option<&serde_json::Value>,
    ) -> Result<i64, GraphError> {
        let meta_value = meta.cloned().unwrap_or(serde_json::json!({}));

        // First ensure both nodes exist (create placeholders if needed)
        self.ensure_node(src_id, "Unknown", None, None).await?;
        self.ensure_node(dst_id, "Unknown", None, None).await?;

        let result = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO graph.edges (src_id, dst_id, edge_type, meta)
            VALUES ($1, $2, $3, $4)
            RETURNING edge_id
            "#,
        )
        .bind(src_id)
        .bind(dst_id)
        .bind(edge_type.as_str())
        .bind(meta_value)
        .fetch_one(&self.pool)
        .await?;

        debug!(
            src_id = src_id,
            dst_id = dst_id,
            edge_type = edge_type.as_str(),
            "Edge added"
        );
        Ok(result)
    }

    /// Check if an edge exists
    pub async fn edge_exists(
        &self,
        src_id: &str,
        dst_id: &str,
        edge_type: EdgeType,
    ) -> Result<bool, GraphError> {
        let result = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM graph.edges
                WHERE src_id = $1 AND dst_id = $2 AND edge_type = $3
            )
            "#,
        )
        .bind(src_id)
        .bind(dst_id)
        .bind(edge_type.as_str())
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    // ========================================================================
    // Traversal Operations per SR-SPEC §1.8.3
    // ========================================================================

    /// Get transitive dependencies of a node
    ///
    /// Per SR-SPEC §1.8.3: Returns all nodes that the given node depends on.
    #[instrument(skip(self))]
    pub async fn get_dependencies(
        &self,
        node_id: &str,
        max_depth: i32,
    ) -> Result<Vec<DependencyEdge>, GraphError> {
        let rows = sqlx::query_as::<_, (String, String, String, i32)>(
            r#"
            SELECT src_id, dst_id, edge_type, depth
            FROM graph.get_dependencies($1, $2)
            "#,
        )
        .bind(node_id)
        .bind(max_depth)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(src_id, dst_id, edge_type, depth)| DependencyEdge {
                src_id,
                dst_id,
                edge_type,
                depth,
            })
            .collect())
    }

    /// Get transitive dependents of a node (impact analysis)
    ///
    /// Per SR-SPEC §1.13.4: Returns all nodes that depend on the given node.
    #[instrument(skip(self))]
    pub async fn get_dependents(
        &self,
        node_id: &str,
        max_depth: i32,
    ) -> Result<Vec<DependencyEdge>, GraphError> {
        let rows = sqlx::query_as::<_, (String, String, String, i32)>(
            r#"
            SELECT dependent_id, dependency_id, edge_type, depth
            FROM graph.get_dependents($1, $2)
            "#,
        )
        .bind(node_id)
        .bind(max_depth)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(
                |(dependent_id, dependency_id, edge_type, depth)| DependencyEdge {
                    src_id: dependent_id,
                    dst_id: dependency_id,
                    edge_type,
                    depth,
                },
            )
            .collect())
    }

    // ========================================================================
    // Staleness Operations per SR-SPEC §1.13
    // ========================================================================

    /// Mark a node as stale
    #[instrument(skip(self))]
    pub async fn mark_stale(
        &self,
        root_kind: &str,
        root_id: &str,
        dependent_kind: &str,
        dependent_id: &str,
        reason: StalenessReason,
        reason_detail: Option<&str>,
        actor_kind: &ActorKind,
        actor_id: &str,
    ) -> Result<String, GraphError> {
        let stale_id = format!("stale_{}", Ulid::new());
        let actor_kind_str = match actor_kind {
            ActorKind::Human => "HUMAN",
            ActorKind::Agent => "AGENT",
            ActorKind::System => "SYSTEM",
        };

        sqlx::query(
            r#"
            INSERT INTO graph.stale_nodes (
                stale_id, root_kind, root_id, dependent_kind, dependent_id,
                reason_code, reason_detail, marked_at, marked_by_kind, marked_by_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), $8, $9)
            "#,
        )
        .bind(&stale_id)
        .bind(root_kind)
        .bind(root_id)
        .bind(dependent_kind)
        .bind(dependent_id)
        .bind(reason.as_str())
        .bind(reason_detail)
        .bind(actor_kind_str)
        .bind(actor_id)
        .execute(&self.pool)
        .await?;

        info!(
            stale_id = stale_id,
            dependent_id = dependent_id,
            reason = reason.as_str(),
            "Node marked as stale"
        );
        Ok(stale_id)
    }

    /// Resolve a staleness marker
    #[instrument(skip(self))]
    pub async fn resolve_staleness(
        &self,
        stale_id: &str,
        resolution_event_id: &str,
    ) -> Result<(), GraphError> {
        let rows_affected = sqlx::query(
            r#"
            UPDATE graph.stale_nodes
            SET resolved_at = NOW(), resolution_event_id = $1
            WHERE stale_id = $2 AND resolved_at IS NULL
            "#,
        )
        .bind(resolution_event_id)
        .bind(stale_id)
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            warn!(
                stale_id = stale_id,
                "Staleness marker not found or already resolved"
            );
        } else {
            info!(stale_id = stale_id, "Staleness resolved");
        }

        Ok(())
    }

    /// Check if a node has unresolved staleness
    pub async fn has_unresolved_staleness(&self, node_id: &str) -> Result<bool, GraphError> {
        let result = sqlx::query_scalar::<_, bool>(r#"SELECT graph.has_unresolved_staleness($1)"#)
            .bind(node_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(result)
    }

    /// Get unresolved staleness markers for a node
    pub async fn get_staleness_markers(
        &self,
        node_id: &str,
    ) -> Result<Vec<StalenessMarker>, GraphError> {
        let rows = sqlx::query_as::<
            _,
            (
                String,
                String,
                String,
                String,
                Option<String>,
                DateTime<Utc>,
            ),
        >(
            r#"
            SELECT stale_id, root_kind, root_id, reason_code, reason_detail, marked_at
            FROM graph.get_staleness_markers($1)
            "#,
        )
        .bind(node_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(
                |(stale_id, root_kind, root_id, reason_code, reason_detail, marked_at)| {
                    StalenessMarker {
                        stale_id,
                        root_kind,
                        root_id,
                        dependent_kind: String::new(), // Not returned by this function
                        dependent_id: node_id.to_string(),
                        reason_code,
                        reason_detail,
                        marked_at,
                        marked_by_kind: String::new(),
                        marked_by_id: String::new(),
                        resolved_at: None,
                        resolution_event_id: None,
                    }
                },
            )
            .collect())
    }

    /// Propagate staleness to dependents
    ///
    /// Per SR-SPEC §1.13.4: When a node becomes stale, all its transitive
    /// dependents should also be marked stale.
    #[instrument(skip(self))]
    pub async fn propagate_staleness(
        &self,
        root_kind: &str,
        root_id: &str,
        reason: StalenessReason,
        reason_detail: Option<&str>,
        actor_kind: &ActorKind,
        actor_id: &str,
        max_depth: i32,
    ) -> Result<Vec<String>, GraphError> {
        // Get all dependents
        let dependents = self.get_dependents(root_id, max_depth).await?;

        let mut stale_ids = Vec::new();

        for dep in dependents {
            // Get the node type for the dependent
            let node = self.get_node(&dep.src_id).await?;
            let dependent_kind = node
                .map(|n| n.node_type)
                .unwrap_or_else(|| "Unknown".to_string());

            let stale_id = self
                .mark_stale(
                    root_kind,
                    root_id,
                    &dependent_kind,
                    &dep.src_id,
                    StalenessReason::DependencyStale,
                    reason_detail,
                    actor_kind,
                    actor_id,
                )
                .await?;

            stale_ids.push(stale_id);
        }

        info!(
            root_id = root_id,
            dependents_marked = stale_ids.len(),
            "Staleness propagated to dependents"
        );

        Ok(stale_ids)
    }

    // ========================================================================
    // Event Processing - Build graph from events
    // ========================================================================

    /// Process an event and update the graph
    #[instrument(skip(self, event), fields(event_id = %event.event_id.as_str(), event_type = %event.event_type))]
    pub async fn process_event(&self, event: &EventEnvelope) -> Result<(), GraphError> {
        // Extract the primary node from the event
        let node_id = &event.stream_id;
        let node_type = infer_node_type(&event.event_type, &event.stream_id);

        // Ensure the primary node exists
        self.ensure_node(node_id, &node_type, None, None).await?;

        // Process typed refs to create edges
        for typed_ref in &event.refs {
            let edge_type = match EdgeType::from_str(&typed_ref.rel) {
                Some(et) => et,
                None => {
                    debug!(rel = typed_ref.rel, "Unknown edge type, using relates_to");
                    EdgeType::RelatesTo
                }
            };

            // Ensure the referenced node exists
            self.ensure_node(&typed_ref.id, &typed_ref.kind, None, Some(&typed_ref.meta))
                .await?;

            // Add edge from current node to referenced node
            if !self.edge_exists(node_id, &typed_ref.id, edge_type).await? {
                self.add_edge(node_id, &typed_ref.id, edge_type, Some(&typed_ref.meta))
                    .await?;
            }
        }

        // Handle staleness-related events
        match event.event_type.as_str() {
            "NodeMarkedStale" => {
                let payload = &event.payload;
                let root_kind = payload["root_kind"].as_str().unwrap_or("");
                let root_id = payload["root_id"].as_str().unwrap_or("");
                let dependent_kind = payload["dependent_kind"].as_str().unwrap_or("");
                let dependent_id = payload["dependent_id"].as_str().unwrap_or("");
                let reason_code = payload["reason_code"].as_str().unwrap_or("MANUAL_MARK");
                let reason_detail = payload["reason_detail"].as_str();

                let reason =
                    StalenessReason::from_str(reason_code).unwrap_or(StalenessReason::ManualMark);

                self.mark_stale(
                    root_kind,
                    root_id,
                    dependent_kind,
                    dependent_id,
                    reason,
                    reason_detail,
                    &event.actor_kind,
                    &event.actor_id,
                )
                .await?;
            }
            "StalenessResolved" => {
                let payload = &event.payload;
                let stale_id = payload["stale_id"].as_str().unwrap_or("");
                self.resolve_staleness(stale_id, event.event_id.as_str())
                    .await?;
            }
            "GovernedArtifactVersionRecorded" => {
                // Propagate staleness when a governed artifact changes
                let payload = &event.payload;
                let artifact_id = payload["artifact_id"].as_str().unwrap_or("");

                if !artifact_id.is_empty() {
                    self.propagate_staleness(
                        "GovernedArtifact",
                        artifact_id,
                        StalenessReason::GovernedArtifactChanged,
                        Some("New version recorded"),
                        &event.actor_kind,
                        &event.actor_id,
                        10,
                    )
                    .await?;
                }
            }
            "OracleSuiteRebased" => {
                // Propagate staleness when an oracle suite is rebased
                let payload = &event.payload;
                let suite_id = payload["oracle_suite_id"].as_str().unwrap_or("");

                if !suite_id.is_empty() {
                    self.propagate_staleness(
                        "OracleSuite",
                        suite_id,
                        StalenessReason::OracleSuiteRebased,
                        Some("Oracle suite rebased"),
                        &event.actor_kind,
                        &event.actor_id,
                        10,
                    )
                    .await?;
                }
            }
            "ExceptionActivated" => {
                // Propagate staleness when an exception is activated
                let exception_id = &event.stream_id;

                self.propagate_staleness(
                    "Exception",
                    exception_id,
                    StalenessReason::ExceptionActivated,
                    Some("Exception activated"),
                    &event.actor_kind,
                    &event.actor_id,
                    10,
                )
                .await?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Rebuild graph from all events
    pub async fn rebuild(&self) -> Result<(), GraphError> {
        info!("Truncating graph tables for rebuild");

        sqlx::query("TRUNCATE TABLE graph.stale_nodes")
            .execute(&self.pool)
            .await?;
        sqlx::query("TRUNCATE TABLE graph.edges CASCADE")
            .execute(&self.pool)
            .await?;
        sqlx::query("TRUNCATE TABLE graph.nodes CASCADE")
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

/// Infer node type from event type and stream ID
fn infer_node_type(event_type: &str, stream_id: &str) -> String {
    match event_type {
        "LoopCreated" | "LoopActivated" | "LoopPaused" | "LoopResumed" | "LoopClosed" => {
            "Loop".to_string()
        }
        "IterationStarted" | "IterationCompleted" | "IterationSummaryRecorded" => {
            "Iteration".to_string()
        }
        "CandidateMaterialized" | "CandidateVerificationComputed" => "Candidate".to_string(),
        "RunStarted" | "RunCompleted" | "EvidenceBundleRecorded" => "Run".to_string(),
        "ApprovalRecorded" => "Approval".to_string(),
        "DecisionRecorded" => "Decision".to_string(),
        "DeviationCreated" | "DeferralCreated" | "WaiverCreated" | "ExceptionActivated"
        | "ExceptionResolved" | "ExceptionExpired" => "Exception".to_string(),
        "FreezeRecordCreated" => "Freeze".to_string(),
        "GovernedArtifactVersionRecorded" => "GovernedArtifact".to_string(),
        "OracleSuiteRegistered"
        | "OracleSuiteUpdated"
        | "OracleSuitePinned"
        | "OracleSuiteRebased" => "OracleSuite".to_string(),
        "WorkSurfaceRecorded" => "WorkSurface".to_string(),
        "ProcedureTemplateSelected" => "ProcedureTemplate".to_string(),
        _ => {
            // Infer from stream_id prefix
            if stream_id.starts_with("loop_") {
                "Loop".to_string()
            } else if stream_id.starts_with("iter_") {
                "Iteration".to_string()
            } else if stream_id.contains("cand_") {
                "Candidate".to_string()
            } else if stream_id.starts_with("run_") {
                "Run".to_string()
            } else if stream_id.starts_with("appr_") {
                "Approval".to_string()
            } else if stream_id.starts_with("dec_") {
                "Decision".to_string()
            } else if stream_id.starts_with("exc_") {
                "Exception".to_string()
            } else if stream_id.starts_with("freeze_") {
                "Freeze".to_string()
            } else {
                "Unknown".to_string()
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_type_conversion() {
        assert_eq!(EdgeType::DependsOn.as_str(), "depends_on");
        assert_eq!(EdgeType::from_str("depends_on"), Some(EdgeType::DependsOn));
        assert_eq!(EdgeType::from_str("invalid"), None);
    }

    #[test]
    fn test_staleness_reason_conversion() {
        assert_eq!(
            StalenessReason::GovernedArtifactChanged.as_str(),
            "GOVERNED_ARTIFACT_CHANGED"
        );
        assert_eq!(
            StalenessReason::from_str("GOVERNED_ARTIFACT_CHANGED"),
            Some(StalenessReason::GovernedArtifactChanged)
        );
        assert_eq!(StalenessReason::from_str("INVALID"), None);
    }

    #[test]
    fn test_infer_node_type() {
        assert_eq!(infer_node_type("LoopCreated", "loop_123"), "Loop");
        assert_eq!(infer_node_type("IterationStarted", "iter_123"), "Iteration");
        assert_eq!(
            infer_node_type("CandidateMaterialized", "sha256:abc|cand_123"),
            "Candidate"
        );
        assert_eq!(infer_node_type("Unknown", "loop_123"), "Loop");
        assert_eq!(infer_node_type("Unknown", "unknown_123"), "Unknown");
    }
}
