//! Context Compilation Rules per SR-SPEC §4.4
//!
//! This module provides deterministic rules for compiling typed refs into a context
//! bundle for workers/oracles. The compilation is guaranteed to be deterministic:
//! given the same ref set and artifact contents, the compiled context bundle is
//! content-hash identical.
//!
//! Key properties:
//! - Deterministic ordering (refs are sorted by kind + id + rel)
//! - Explicit handling of restricted/redacted material
//! - Compilation failures are deterministic and explainable

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

use crate::entities::{ActorId, ContentHash, TypedRef};
use crate::errors::DomainError;

/// Context bundle - the deterministic output of context compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBundle {
    /// Content hash of the compiled bundle (for verification)
    pub content_hash: ContentHash,

    /// Ordered list of context items
    pub items: Vec<ContextItem>,

    /// Compilation metadata
    pub metadata: ContextMetadata,

    /// Redaction records (for audit)
    pub redactions: Vec<RedactionRecord>,
}

/// Individual item within a context bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextItem {
    /// Source ref that produced this item
    pub source_ref: TypedRef,

    /// Resolved content hash
    pub content_hash: ContentHash,

    /// Order in which this item should be presented
    pub order: u32,

    /// Classification for handling
    pub classification: ItemClassification,
}

/// Classification of context items per SR-SPEC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemClassification {
    /// Unrestricted - can be included in full
    Public,
    /// Internal - can be included but with restricted distribution
    Internal,
    /// Restricted - must be redacted or excluded
    Restricted,
    /// Confidential - must not be included
    Confidential,
}

/// Metadata about the context compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMetadata {
    /// Total number of refs processed
    pub total_refs: usize,

    /// Number of items included
    pub items_included: usize,

    /// Number of items redacted
    pub items_redacted: usize,

    /// Compilation timestamp (deterministic from input)
    pub compiled_at: chrono::DateTime<chrono::Utc>,

    /// Compiler version for reproducibility
    pub compiler_version: String,
}

/// Record of a redaction for audit purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedactionRecord {
    /// Original ref that was redacted
    pub original_ref: TypedRef,

    /// Reason for redaction
    pub reason: RedactionReason,

    /// Redacted content hash (for verification that redaction was applied)
    pub redacted_hash: ContentHash,
}

/// Reasons for redaction per SR-SPEC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RedactionReason {
    /// Content is classified as restricted
    ClassificationRestricted,
    /// Content contains sensitive PII
    ContainsPii,
    /// Content contains credentials or secrets
    ContainsSecrets,
    /// Content is embargoed until a future date
    Embargoed,
    /// Content was explicitly excluded by policy
    PolicyExclusion,
}

/// Context compiler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
    /// Maximum number of items to include
    pub max_items: usize,

    /// Maximum total size in bytes
    pub max_total_bytes: usize,

    /// Classifications to redact (anything >= this level)
    pub redaction_threshold: ItemClassification,

    /// Whether to include metadata refs
    pub include_metadata: bool,

    /// Compiler version
    pub version: String,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            max_items: 100,
            max_total_bytes: 10 * 1024 * 1024, // 10 MB
            redaction_threshold: ItemClassification::Restricted,
            include_metadata: true,
            version: "1.0.0".to_string(),
        }
    }
}

/// Context compiler - compiles refs into deterministic context bundles
pub struct ContextCompiler {
    config: CompilerConfig,
}

impl ContextCompiler {
    /// Create a new context compiler with default config
    pub fn new() -> Self {
        Self {
            config: CompilerConfig::default(),
        }
    }

    /// Create a context compiler with custom config
    pub fn with_config(config: CompilerConfig) -> Self {
        Self { config }
    }

    /// Compile a set of typed refs into a context bundle
    ///
    /// This method is deterministic: given the same refs and content resolver,
    /// it will produce byte-identical output.
    ///
    /// # Arguments
    /// * `refs` - The typed refs to compile
    /// * `timestamp` - Deterministic timestamp for the compilation
    /// * `resolve_content` - Function to resolve a ref to its content and classification
    ///
    /// # Returns
    /// A deterministic context bundle or an error
    pub fn compile<F>(
        &self,
        refs: &[TypedRef],
        timestamp: chrono::DateTime<chrono::Utc>,
        resolve_content: F,
    ) -> Result<ContextBundle, DomainError>
    where
        F: Fn(&TypedRef) -> Result<(ContentHash, ItemClassification), DomainError>,
    {
        // Step 1: Sort refs deterministically
        let sorted_refs = self.sort_refs(refs);

        // Step 2: Resolve and classify each ref
        let mut items = Vec::new();
        let mut redactions = Vec::new();
        let mut order = 0u32;

        for typed_ref in &sorted_refs {
            let (content_hash, classification) = resolve_content(typed_ref)?;

            // Check if this should be redacted
            if self.should_redact(classification) {
                redactions.push(RedactionRecord {
                    original_ref: typed_ref.clone(),
                    reason: RedactionReason::ClassificationRestricted,
                    redacted_hash: self.compute_redacted_hash(&content_hash),
                });
            } else {
                items.push(ContextItem {
                    source_ref: typed_ref.clone(),
                    content_hash,
                    order,
                    classification,
                });
                order += 1;
            }

            // Check limits
            if items.len() >= self.config.max_items {
                break;
            }
        }

        // Step 3: Compute deterministic content hash of the bundle
        let bundle_hash = self.compute_bundle_hash(&items, &redactions);

        // Step 4: Build metadata
        let metadata = ContextMetadata {
            total_refs: refs.len(),
            items_included: items.len(),
            items_redacted: redactions.len(),
            compiled_at: timestamp,
            compiler_version: self.config.version.clone(),
        };

        Ok(ContextBundle {
            content_hash: bundle_hash,
            items,
            metadata,
            redactions,
        })
    }

    /// Sort refs deterministically by (kind, id, rel)
    fn sort_refs(&self, refs: &[TypedRef]) -> Vec<TypedRef> {
        let mut sorted: Vec<TypedRef> = refs.to_vec();
        sorted.sort_by(|a, b| {
            match a.kind.cmp(&b.kind) {
                std::cmp::Ordering::Equal => match a.id.cmp(&b.id) {
                    std::cmp::Ordering::Equal => a.rel.cmp(&b.rel),
                    other => other,
                },
                other => other,
            }
        });
        sorted
    }

    /// Check if a classification level requires redaction
    fn should_redact(&self, classification: ItemClassification) -> bool {
        match (classification, self.config.redaction_threshold) {
            (ItemClassification::Public, _) => false,
            (ItemClassification::Internal, ItemClassification::Public) => true,
            (ItemClassification::Internal, _) => false,
            (ItemClassification::Restricted, ItemClassification::Confidential) => false,
            (ItemClassification::Restricted, _) => true,
            (ItemClassification::Confidential, _) => true,
        }
    }

    /// Compute a redacted placeholder hash
    fn compute_redacted_hash(&self, original_hash: &ContentHash) -> ContentHash {
        let mut hasher = Sha256::new();
        hasher.update(b"REDACTED:");
        hasher.update(original_hash.as_str().as_bytes());
        let result = hasher.finalize();
        ContentHash::new(&hex::encode(result))
    }

    /// Compute deterministic bundle hash
    fn compute_bundle_hash(
        &self,
        items: &[ContextItem],
        redactions: &[RedactionRecord],
    ) -> ContentHash {
        let mut hasher = Sha256::new();

        // Include items in order
        for item in items {
            hasher.update(item.source_ref.kind.as_bytes());
            hasher.update(b":");
            hasher.update(item.source_ref.id.as_bytes());
            hasher.update(b":");
            hasher.update(item.content_hash.as_str().as_bytes());
            hasher.update(b"\n");
        }

        // Include redaction records for completeness
        hasher.update(b"REDACTIONS:");
        for redaction in redactions {
            hasher.update(redaction.original_ref.kind.as_bytes());
            hasher.update(b":");
            hasher.update(redaction.original_ref.id.as_bytes());
            hasher.update(b"\n");
        }

        let result = hasher.finalize();
        ContentHash::new(&hex::encode(result))
    }
}

impl Default for ContextCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Selection rules for building ref sets per SR-SPEC §4.4
pub struct RefSelector;

impl RefSelector {
    /// Select refs for a work unit context
    ///
    /// Per SR-SPEC §4.4, the selection follows priority:
    /// 1. Direct dependencies (depends_on edges)
    /// 2. Governing documents (governed_by edges)
    /// 3. Supporting evidence (supported_by edges)
    pub fn select_for_work_unit(
        direct_deps: &[TypedRef],
        governing_docs: &[TypedRef],
        evidence: &[TypedRef],
    ) -> Vec<TypedRef> {
        let mut result = Vec::new();

        // Add direct dependencies first (highest priority)
        for r in direct_deps {
            if !result.iter().any(|x: &TypedRef| x.id == r.id && x.kind == r.kind) {
                result.push(r.clone());
            }
        }

        // Add governing documents
        for r in governing_docs {
            if !result.iter().any(|x: &TypedRef| x.id == r.id && x.kind == r.kind) {
                result.push(r.clone());
            }
        }

        // Add evidence
        for r in evidence {
            if !result.iter().any(|x: &TypedRef| x.id == r.id && x.kind == r.kind) {
                result.push(r.clone());
            }
        }

        result
    }

    /// Build a dependency-ordered ref list
    ///
    /// Given a dependency graph (as adjacency list), returns refs in
    /// topological order (dependencies before dependents).
    pub fn topological_sort(
        refs: &[TypedRef],
        dependencies: &BTreeMap<String, Vec<String>>, // id -> [dependency_ids]
    ) -> Result<Vec<TypedRef>, DomainError> {
        let ref_map: BTreeMap<String, &TypedRef> =
            refs.iter().map(|r| (r.id.clone(), r)).collect();

        let mut in_degree: BTreeMap<String, usize> = BTreeMap::new();
        let mut adj: BTreeMap<String, Vec<String>> = BTreeMap::new();

        // Initialize
        for r in refs {
            in_degree.entry(r.id.clone()).or_insert(0);
            adj.entry(r.id.clone()).or_default();
        }

        // Build adjacency and in-degree
        for (id, deps) in dependencies {
            for dep in deps {
                if ref_map.contains_key(dep) && ref_map.contains_key(id) {
                    adj.entry(dep.clone()).or_default().push(id.clone());
                    *in_degree.entry(id.clone()).or_default() += 1;
                }
            }
        }

        // Kahn's algorithm
        let mut queue: Vec<String> = in_degree
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(id, _)| id.clone())
            .collect();
        queue.sort(); // Deterministic ordering

        let mut result = Vec::new();

        while let Some(current) = queue.pop() {
            if let Some(r) = ref_map.get(&current) {
                result.push((*r).clone());
            }

            if let Some(neighbors) = adj.get(&current) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(neighbor.clone());
                            queue.sort(); // Maintain deterministic order
                        }
                    }
                }
            }
        }

        // Check for cycles
        if result.len() != refs.len() {
            return Err(DomainError::InvariantViolation {
                invariant: "Dependency graph contains a cycle".to_string(),
            });
        }

        Ok(result)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn make_ref(kind: &str, id: &str, rel: &str) -> TypedRef {
        TypedRef {
            kind: kind.to_string(),
            id: id.to_string(),
            rel: rel.to_string(),
            meta: serde_json::Value::Null,
        }
    }

    #[test]
    fn test_deterministic_ordering() {
        let refs = vec![
            make_ref("Doc", "c", "depends_on"),
            make_ref("Doc", "a", "depends_on"),
            make_ref("Code", "b", "about"),
            make_ref("Doc", "b", "governed_by"),
        ];

        let compiler = ContextCompiler::new();
        let sorted = compiler.sort_refs(&refs);

        // Should be sorted by (kind, id, rel)
        assert_eq!(sorted[0].kind, "Code");
        assert_eq!(sorted[0].id, "b");
        assert_eq!(sorted[1].kind, "Doc");
        assert_eq!(sorted[1].id, "a");
        assert_eq!(sorted[2].kind, "Doc");
        assert_eq!(sorted[2].id, "b");
        assert_eq!(sorted[3].kind, "Doc");
        assert_eq!(sorted[3].id, "c");
    }

    #[test]
    fn test_compile_deterministic() {
        let refs = vec![
            make_ref("Doc", "SR-SPEC", "depends_on"),
            make_ref("Doc", "SR-CONTRACT", "governed_by"),
        ];

        let compiler = ContextCompiler::new();
        let timestamp = Utc::now();

        // Mock resolver
        let resolve = |_: &TypedRef| {
            Ok((ContentHash::new("abc123"), ItemClassification::Public))
        };

        let bundle1 = compiler.compile(&refs, timestamp, resolve).unwrap();
        let bundle2 = compiler.compile(&refs, timestamp, resolve).unwrap();

        // Same input should produce same hash
        assert_eq!(bundle1.content_hash.as_str(), bundle2.content_hash.as_str());
    }

    #[test]
    fn test_redaction() {
        let refs = vec![
            make_ref("Doc", "public-doc", "depends_on"),
            make_ref("Secret", "api-key", "contains"),
        ];

        let compiler = ContextCompiler::new();
        let timestamp = Utc::now();

        let resolve = |r: &TypedRef| {
            if r.kind == "Secret" {
                Ok((ContentHash::new("secret123"), ItemClassification::Confidential))
            } else {
                Ok((ContentHash::new("public456"), ItemClassification::Public))
            }
        };

        let bundle = compiler.compile(&refs, timestamp, resolve).unwrap();

        assert_eq!(bundle.items.len(), 1);
        assert_eq!(bundle.redactions.len(), 1);
        assert_eq!(bundle.redactions[0].original_ref.kind, "Secret");
    }

    #[test]
    fn test_ref_selection() {
        let deps = vec![make_ref("Doc", "A", "depends_on")];
        let governing = vec![
            make_ref("Doc", "B", "governed_by"),
            make_ref("Doc", "A", "governed_by"), // Duplicate
        ];
        let evidence = vec![make_ref("Evidence", "C", "supported_by")];

        let selected = RefSelector::select_for_work_unit(&deps, &governing, &evidence);

        // Should have 3 unique refs (duplicate A excluded)
        assert_eq!(selected.len(), 3);
        assert_eq!(selected[0].id, "A");
        assert_eq!(selected[1].id, "B");
        assert_eq!(selected[2].id, "C");
    }

    #[test]
    fn test_topological_sort() {
        let refs = vec![
            make_ref("Doc", "A", "depends_on"),
            make_ref("Doc", "B", "depends_on"),
            make_ref("Doc", "C", "depends_on"),
        ];

        let mut deps = BTreeMap::new();
        deps.insert("B".to_string(), vec!["A".to_string()]); // B depends on A
        deps.insert("C".to_string(), vec!["B".to_string()]); // C depends on B

        let sorted = RefSelector::topological_sort(&refs, &deps).unwrap();

        // A should come before B, B before C
        let a_pos = sorted.iter().position(|r| r.id == "A").unwrap();
        let b_pos = sorted.iter().position(|r| r.id == "B").unwrap();
        let c_pos = sorted.iter().position(|r| r.id == "C").unwrap();

        assert!(a_pos < b_pos);
        assert!(b_pos < c_pos);
    }

    #[test]
    fn test_topological_sort_cycle_detection() {
        let refs = vec![
            make_ref("Doc", "A", "depends_on"),
            make_ref("Doc", "B", "depends_on"),
        ];

        let mut deps = BTreeMap::new();
        deps.insert("A".to_string(), vec!["B".to_string()]);
        deps.insert("B".to_string(), vec!["A".to_string()]); // Cycle!

        let result = RefSelector::topological_sort(&refs, &deps);
        assert!(result.is_err());
    }
}
