//! PostgreSQL Oracle Suite Registry Adapter
//!
//! V8-1: Implements OracleSuiteRegistryPort for persistent oracle suite storage.
//! Per SR-CONTRACT C-OR-2: Runs MUST pin oracle suite identity at start.

use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, PgPool, Row};
use sr_ports::{
    OracleSuiteRecord, OracleSuiteRegistryError, OracleSuiteRegistryPort, OracleSuiteStatus,
    RegisterSuiteInput, SuiteFilter,
};
use tracing::{debug, error, info, instrument};

/// PostgreSQL-backed oracle suite registry
///
/// Implements persistent storage for oracle suite definitions per SR-PLAN-V8 V8-1.
pub struct PostgresOracleSuiteRegistry {
    pool: PgPool,
}

impl PostgresOracleSuiteRegistry {
    /// Create a new PostgreSQL oracle suite registry with an existing pool
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Connect to PostgreSQL and create a new oracle suite registry
    pub async fn connect(database_url: &str) -> Result<Self, OracleSuiteRegistryError> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| OracleSuiteRegistryError::ConnectionError {
                message: e.to_string(),
            })?;
        Ok(Self { pool })
    }

    /// Get a reference to the underlying pool (for testing/migrations)
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

impl OracleSuiteRegistryPort for PostgresOracleSuiteRegistry {
    /// Register a new oracle suite
    #[instrument(skip(self, input), fields(suite_id = %input.suite_id))]
    async fn register(
        &self,
        input: RegisterSuiteInput,
    ) -> Result<OracleSuiteRecord, OracleSuiteRegistryError> {
        // Check for existing suite with same ID
        let existing = sqlx::query("SELECT suite_id FROM proj.oracle_suites WHERE suite_id = $1")
            .bind(&input.suite_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| OracleSuiteRegistryError::ConnectionError {
                message: e.to_string(),
            })?;

        if existing.is_some() {
            return Err(OracleSuiteRegistryError::AlreadyExists {
                suite_id: input.suite_id,
            });
        }

        // Check for existing suite with same hash
        let hash_conflict =
            sqlx::query("SELECT suite_id FROM proj.oracle_suites WHERE suite_hash = $1")
                .bind(&input.suite_hash)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| OracleSuiteRegistryError::ConnectionError {
                    message: e.to_string(),
                })?;

        if hash_conflict.is_some() {
            return Err(OracleSuiteRegistryError::HashConflict {
                suite_hash: input.suite_hash,
            });
        }

        // Insert the new suite
        let row = sqlx::query(
            r#"
            INSERT INTO proj.oracle_suites (
                suite_id, suite_hash, oci_image, oci_image_digest,
                environment_constraints, oracles, metadata,
                registered_by_kind, registered_by_id, status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'active')
            RETURNING
                suite_id, suite_hash, oci_image, oci_image_digest,
                environment_constraints, oracles, metadata,
                registered_at, registered_by_kind, registered_by_id, status
            "#,
        )
        .bind(&input.suite_id)
        .bind(&input.suite_hash)
        .bind(&input.oci_image)
        .bind(&input.oci_image_digest)
        .bind(&input.environment_constraints)
        .bind(&input.oracles)
        .bind(&input.metadata)
        .bind(&input.actor_kind)
        .bind(&input.actor_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to insert oracle suite");
            OracleSuiteRegistryError::ConnectionError {
                message: e.to_string(),
            }
        })?;

        let record = row_to_oracle_suite_record(&row)?;

        info!(
            suite_id = %record.suite_id,
            suite_hash = %record.suite_hash,
            "Oracle suite registered"
        );

        Ok(record)
    }

    /// Get an oracle suite by ID
    #[instrument(skip(self), fields(suite_id = %suite_id))]
    async fn get(
        &self,
        suite_id: &str,
    ) -> Result<Option<OracleSuiteRecord>, OracleSuiteRegistryError> {
        let row = sqlx::query(
            r#"
            SELECT
                suite_id, suite_hash, oci_image, oci_image_digest,
                environment_constraints, oracles, metadata,
                registered_at, registered_by_kind, registered_by_id, status
            FROM proj.oracle_suites
            WHERE suite_id = $1
            "#,
        )
        .bind(suite_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| OracleSuiteRegistryError::ConnectionError {
            message: e.to_string(),
        })?;

        match row {
            Some(r) => Ok(Some(row_to_oracle_suite_record(&r)?)),
            None => Ok(None),
        }
    }

    /// Get an oracle suite by its content hash
    #[instrument(skip(self), fields(suite_hash = %suite_hash))]
    async fn get_by_hash(
        &self,
        suite_hash: &str,
    ) -> Result<Option<OracleSuiteRecord>, OracleSuiteRegistryError> {
        let row = sqlx::query(
            r#"
            SELECT
                suite_id, suite_hash, oci_image, oci_image_digest,
                environment_constraints, oracles, metadata,
                registered_at, registered_by_kind, registered_by_id, status
            FROM proj.oracle_suites
            WHERE suite_hash = $1
            "#,
        )
        .bind(suite_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| OracleSuiteRegistryError::ConnectionError {
            message: e.to_string(),
        })?;

        match row {
            Some(r) => Ok(Some(row_to_oracle_suite_record(&r)?)),
            None => Ok(None),
        }
    }

    /// List oracle suites with optional filtering
    #[instrument(skip(self))]
    async fn list(
        &self,
        filter: SuiteFilter,
    ) -> Result<Vec<OracleSuiteRecord>, OracleSuiteRegistryError> {
        let limit = filter.limit.unwrap_or(100) as i64;

        let rows = match filter.status {
            Some(status) => {
                sqlx::query(
                    r#"
                    SELECT
                        suite_id, suite_hash, oci_image, oci_image_digest,
                        environment_constraints, oracles, metadata,
                        registered_at, registered_by_kind, registered_by_id, status
                    FROM proj.oracle_suites
                    WHERE status = $1
                    ORDER BY registered_at DESC
                    LIMIT $2
                    "#,
                )
                .bind(status.to_string())
                .bind(limit)
                .fetch_all(&self.pool)
                .await
            }
            None => {
                sqlx::query(
                    r#"
                    SELECT
                        suite_id, suite_hash, oci_image, oci_image_digest,
                        environment_constraints, oracles, metadata,
                        registered_at, registered_by_kind, registered_by_id, status
                    FROM proj.oracle_suites
                    ORDER BY registered_at DESC
                    LIMIT $1
                    "#,
                )
                .bind(limit)
                .fetch_all(&self.pool)
                .await
            }
        }
        .map_err(|e| OracleSuiteRegistryError::ConnectionError {
            message: e.to_string(),
        })?;

        rows.iter().map(row_to_oracle_suite_record).collect()
    }

    /// Deprecate an oracle suite (soft delete)
    #[instrument(skip(self), fields(suite_id = %suite_id))]
    async fn deprecate(
        &self,
        suite_id: &str,
        _actor_kind: &str,
        _actor_id: &str,
    ) -> Result<(), OracleSuiteRegistryError> {
        let result = sqlx::query(
            r#"
            UPDATE proj.oracle_suites
            SET status = 'deprecated'
            WHERE suite_id = $1 AND status = 'active'
            "#,
        )
        .bind(suite_id)
        .execute(&self.pool)
        .await
        .map_err(|e| OracleSuiteRegistryError::ConnectionError {
            message: e.to_string(),
        })?;

        if result.rows_affected() == 0 {
            // Check if it exists at all
            let exists = self.get(suite_id).await?;
            if exists.is_none() {
                return Err(OracleSuiteRegistryError::NotFound {
                    suite_id: suite_id.to_string(),
                });
            }
            // Suite exists but is already deprecated or archived - that's fine
            debug!(suite_id = %suite_id, "Suite already deprecated or archived");
        } else {
            info!(suite_id = %suite_id, "Oracle suite deprecated");
        }

        Ok(())
    }
}

/// Convert a database row to an OracleSuiteRecord
fn row_to_oracle_suite_record(row: &PgRow) -> Result<OracleSuiteRecord, OracleSuiteRegistryError> {
    let suite_id: String = row.get("suite_id");
    let suite_hash: String = row.get("suite_hash");
    let oci_image: String = row.get("oci_image");
    let oci_image_digest: String = row.get("oci_image_digest");
    let environment_constraints: serde_json::Value = row.get("environment_constraints");
    let oracles: serde_json::Value = row.get("oracles");
    let metadata: serde_json::Value = row.get("metadata");
    let registered_at: DateTime<Utc> = row.get("registered_at");
    let registered_by_kind: String = row.get("registered_by_kind");
    let registered_by_id: String = row.get("registered_by_id");
    let status_str: String = row.get("status");

    let status = status_str
        .parse::<OracleSuiteStatus>()
        .map_err(|e| OracleSuiteRegistryError::SerializationError { message: e })?;

    Ok(OracleSuiteRecord {
        suite_id,
        suite_hash,
        oci_image,
        oci_image_digest,
        environment_constraints,
        oracles,
        metadata,
        registered_at,
        registered_by_kind,
        registered_by_id,
        status,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_parsing() {
        assert_eq!(
            "active".parse::<OracleSuiteStatus>().unwrap(),
            OracleSuiteStatus::Active
        );
        assert_eq!(
            "deprecated".parse::<OracleSuiteStatus>().unwrap(),
            OracleSuiteStatus::Deprecated
        );
        assert_eq!(
            "archived".parse::<OracleSuiteStatus>().unwrap(),
            OracleSuiteStatus::Archived
        );
        assert!("invalid".parse::<OracleSuiteStatus>().is_err());
    }

    #[test]
    fn test_status_display() {
        assert_eq!(OracleSuiteStatus::Active.to_string(), "active");
        assert_eq!(OracleSuiteStatus::Deprecated.to_string(), "deprecated");
        assert_eq!(OracleSuiteStatus::Archived.to_string(), "archived");
    }

    #[test]
    fn test_suite_filter_default() {
        let filter = SuiteFilter::default();
        assert!(filter.status.is_none());
        assert!(filter.limit.is_none());
    }
}
