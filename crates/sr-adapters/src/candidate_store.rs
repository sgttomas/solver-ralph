//! Candidate workspace materializer for oracle execution
//!
//! Per SR-PLAN-V8 §V8-2: The Oracle Execution Worker needs to materialize
//! candidate content to a temporary workspace directory for oracle execution.

use std::future::Future;
use std::path::PathBuf;
use thiserror::Error;
use tracing::{debug, info};

/// Error type for workspace operations
#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("Candidate not found: {candidate_id}")]
    CandidateNotFound { candidate_id: String },

    #[error("Failed to create workspace directory: {reason}")]
    DirectoryCreationFailed { reason: String },

    #[error("Failed to fetch content: {reason}")]
    ContentFetchFailed { reason: String },

    #[error("Failed to write content: {reason}")]
    ContentWriteFailed { reason: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Temporary workspace for oracle execution
///
/// Contains the materialized candidate content in a temporary directory.
/// The directory is cleaned up when the TempWorkspace is dropped.
#[derive(Debug)]
pub struct TempWorkspace {
    /// Path to the workspace directory
    pub path: PathBuf,
    /// Candidate ID this workspace was created for
    pub candidate_id: String,
    /// Content hash of the candidate
    pub content_hash: String,
    /// Whether to clean up on drop (disabled in test mode)
    cleanup_on_drop: bool,
}

impl TempWorkspace {
    /// Create a new TempWorkspace
    pub fn new(
        path: PathBuf,
        candidate_id: String,
        content_hash: String,
        cleanup_on_drop: bool,
    ) -> Self {
        Self {
            path,
            candidate_id,
            content_hash,
            cleanup_on_drop,
        }
    }
}

impl Drop for TempWorkspace {
    fn drop(&mut self) {
        if self.cleanup_on_drop && self.path.exists() {
            if let Err(e) = std::fs::remove_dir_all(&self.path) {
                tracing::warn!(
                    path = %self.path.display(),
                    error = %e,
                    "Failed to clean up workspace directory"
                );
            } else {
                debug!(path = %self.path.display(), "Cleaned up workspace directory");
            }
        }
    }
}

/// Trait for materializing candidate content to a workspace
pub trait CandidateWorkspace: Send + Sync {
    /// Materialize candidate content to a temporary workspace directory
    ///
    /// Returns a TempWorkspace containing the path to the materialized content.
    /// The workspace will be cleaned up when the TempWorkspace is dropped.
    fn materialize(
        &self,
        candidate_id: &str,
    ) -> impl Future<Output = Result<TempWorkspace, WorkspaceError>> + Send;
}

/// Configuration for the simple candidate workspace
#[derive(Debug, Clone)]
pub struct CandidateWorkspaceConfig {
    /// Base path for workspace directories
    pub base_path: PathBuf,
    /// Whether to clean up workspaces on drop
    pub cleanup_on_drop: bool,
    /// Test mode: use placeholder content
    pub test_mode: bool,
}

impl Default for CandidateWorkspaceConfig {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("/tmp/sr-workspaces"),
            cleanup_on_drop: true,
            test_mode: false,
        }
    }
}

/// Simple candidate workspace implementation
///
/// For V8-2, this creates workspace directories with placeholder content.
/// Full content fetching from MinIO can be added in a future enhancement.
pub struct SimpleCandidateWorkspace {
    config: CandidateWorkspaceConfig,
}

impl SimpleCandidateWorkspace {
    /// Create a new SimpleCandidateWorkspace
    pub fn new(config: CandidateWorkspaceConfig) -> Self {
        Self { config }
    }

    /// Create a workspace for test mode
    pub fn test_mode() -> Self {
        Self::new(CandidateWorkspaceConfig {
            base_path: PathBuf::from("/tmp/sr-test-workspaces"),
            cleanup_on_drop: false,
            test_mode: true,
        })
    }
}

impl CandidateWorkspace for SimpleCandidateWorkspace {
    async fn materialize(&self, candidate_id: &str) -> Result<TempWorkspace, WorkspaceError> {
        // Create workspace directory path
        let workspace_path = self.config.base_path.join(candidate_id);

        // Create the directory if it doesn't exist
        if !workspace_path.exists() {
            std::fs::create_dir_all(&workspace_path).map_err(|e| {
                WorkspaceError::DirectoryCreationFailed {
                    reason: e.to_string(),
                }
            })?;
        }

        if self.config.test_mode {
            // In test mode, create a placeholder file
            let placeholder_path = workspace_path.join("PLACEHOLDER.md");
            if !placeholder_path.exists() {
                std::fs::write(
                    &placeholder_path,
                    format!(
                        "# Test Workspace\n\nCandidate ID: {}\n\nThis is a placeholder workspace for testing.\n",
                        candidate_id
                    ),
                )?;
            }
        }

        // Generate a placeholder content hash
        // In production, this would come from the CandidateProjection
        let content_hash = format!("sha256:placeholder_{}", candidate_id.replace(':', "_"));

        info!(
            candidate_id = %candidate_id,
            workspace_path = %workspace_path.display(),
            "Materialized candidate workspace"
        );

        Ok(TempWorkspace::new(
            workspace_path,
            candidate_id.to_string(),
            content_hash,
            self.config.cleanup_on_drop,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_workspace_materialization() {
        let workspace = SimpleCandidateWorkspace::test_mode();
        let candidate_id = "cand:test_12345";

        let result = workspace.materialize(candidate_id).await;
        assert!(result.is_ok());

        let temp_ws = result.unwrap();
        assert_eq!(temp_ws.candidate_id, candidate_id);
        assert!(temp_ws.path.exists());
        assert!(temp_ws.path.join("PLACEHOLDER.md").exists());
    }

    #[tokio::test]
    async fn test_workspace_path_structure() {
        let config = CandidateWorkspaceConfig {
            base_path: PathBuf::from("/tmp/sr-test-ws-path"),
            cleanup_on_drop: false,
            test_mode: true,
        };
        let workspace = SimpleCandidateWorkspace::new(config);
        let candidate_id = "cand:abc123";

        let temp_ws = workspace.materialize(candidate_id).await.unwrap();
        assert_eq!(
            temp_ws.path,
            PathBuf::from("/tmp/sr-test-ws-path/cand:abc123")
        );
    }
}
