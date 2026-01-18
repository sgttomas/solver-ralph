//! Governed Artifacts Manifest (V11-6, D-08)
//!
//! Per SR-PLAN-V11 and SR-CONTRACT C-CTX-1:
//! - GovernedArtifact refs must be content-addressed for immutability
//! - The manifest is computed at API startup and included in IterationStarted.refs[]
//! - This enables verification that iterations operated under the correct governance

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sr_domain::TypedRef;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// A reference to a governed artifact (governance document)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernedArtifactRef {
    /// Document identifier (e.g., "SR-DIRECTIVE", "SR-CONTRACT")
    pub doc_id: String,
    /// Path relative to docs root
    pub path: String,
    /// SHA-256 content hash for immutability verification
    pub content_hash: String,
    /// Optional version from YAML frontmatter
    pub version: Option<String>,
    /// Type key for categorization (e.g., "governance.dev_directive")
    pub type_key: String,
}

/// Manifest of all governed artifacts computed at startup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernedManifest {
    /// List of governed artifacts
    pub artifacts: Vec<GovernedArtifactRef>,
    /// When the manifest was computed
    pub computed_at: DateTime<Utc>,
    /// Combined hash of all artifacts for quick comparison
    pub manifest_hash: String,
}

impl GovernedManifest {
    /// Compute the manifest from the docs directory
    ///
    /// Reads governance documents (SR-*.md in docs/) and computes
    /// content hashes for each. The resulting manifest provides
    /// immutable references per SR-CONTRACT C-CTX-1.
    pub fn compute(docs_path: &Path) -> Self {
        let mut artifacts = Vec::new();

        // Define the governed documents to include
        let governed_docs = [
            (
                "SR-DIRECTIVE",
                "charter/SR-DIRECTIVE.md",
                "governance.dev_directive",
            ),
            (
                "SR-CONTRACT",
                "charter/SR-CONTRACT.md",
                "governance.contract",
            ),
            ("SR-SPEC", "platform/SR-SPEC.md", "governance.specification"),
            ("SR-LOOPS", "platform/SR-LOOPS.md", "governance.loops_model"),
            ("SR-PLAN-V11", "planning/SR-PLAN-V11.md", "governance.plan"),
        ];

        for (doc_id, rel_path, type_key) in governed_docs {
            let full_path = docs_path.join(rel_path);

            if full_path.exists() {
                match fs::read_to_string(&full_path) {
                    Ok(content) => {
                        let content_hash = compute_content_hash(&content);
                        let version = extract_version_from_frontmatter(&content);

                        artifacts.push(GovernedArtifactRef {
                            doc_id: doc_id.to_string(),
                            path: rel_path.to_string(),
                            content_hash,
                            version,
                            type_key: type_key.to_string(),
                        });

                        debug!(
                            doc_id = %doc_id,
                            path = %rel_path,
                            "Added governed artifact to manifest"
                        );
                    }
                    Err(e) => {
                        warn!(
                            doc_id = %doc_id,
                            path = %full_path.display(),
                            error = %e,
                            "Failed to read governed artifact"
                        );
                    }
                }
            } else {
                debug!(
                    doc_id = %doc_id,
                    path = %full_path.display(),
                    "Governed artifact not found, skipping"
                );
            }
        }

        // Compute manifest hash from all artifact hashes
        let manifest_hash = compute_manifest_hash(&artifacts);
        let computed_at = Utc::now();

        info!(
            artifact_count = artifacts.len(),
            manifest_hash = %manifest_hash,
            "Computed governed artifacts manifest"
        );

        Self {
            artifacts,
            computed_at,
            manifest_hash,
        }
    }

    /// Convert artifacts to TypedRef format for inclusion in IterationStarted.refs[]
    ///
    /// Per SR-PLAN-V11-CONSISTENCY-REVIEW, the corrected schema is:
    /// - kind: "GovernedArtifact"
    /// - id: doc_id (e.g., "SR-DIRECTIVE")
    /// - rel: "depends_on" (NOT "governed_by")
    /// - meta: { content_hash, version, type_key }
    pub fn to_typed_refs(&self) -> Vec<TypedRef> {
        self.artifacts
            .iter()
            .map(|artifact| TypedRef {
                kind: "GovernedArtifact".to_string(),
                id: artifact.doc_id.clone(),
                rel: "depends_on".to_string(),
                meta: serde_json::json!({
                    "content_hash": artifact.content_hash,
                    "version": artifact.version,
                    "type_key": artifact.type_key,
                }),
            })
            .collect()
    }

    /// Get a specific artifact by doc_id
    pub fn get_artifact(&self, doc_id: &str) -> Option<&GovernedArtifactRef> {
        self.artifacts.iter().find(|a| a.doc_id == doc_id)
    }
}

impl Default for GovernedManifest {
    fn default() -> Self {
        Self {
            artifacts: Vec::new(),
            computed_at: Utc::now(),
            manifest_hash: "sha256:0".repeat(64),
        }
    }
}

/// Compute SHA-256 hash of content
fn compute_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

/// Compute combined hash of all artifacts
fn compute_manifest_hash(artifacts: &[GovernedArtifactRef]) -> String {
    let mut hasher = Sha256::new();

    // Sort by doc_id for determinism
    let mut sorted: Vec<_> = artifacts.iter().collect();
    sorted.sort_by(|a, b| a.doc_id.cmp(&b.doc_id));

    for artifact in sorted {
        hasher.update(artifact.doc_id.as_bytes());
        hasher.update(b":");
        hasher.update(artifact.content_hash.as_bytes());
        hasher.update(b"\n");
    }

    format!("sha256:{}", hex::encode(hasher.finalize()))
}

/// Extract version from YAML frontmatter if present
fn extract_version_from_frontmatter(content: &str) -> Option<String> {
    // Simple frontmatter parsing - look for version in YAML block
    if !content.starts_with("---") {
        return None;
    }

    let end = content[3..].find("---")?;
    let frontmatter = &content[3..3 + end];

    for line in frontmatter.lines() {
        let line = line.trim();
        if line.starts_with("version:") {
            let version = line.strip_prefix("version:")?.trim();
            // Remove quotes if present
            let version = version.trim_matches('"').trim_matches('\'');
            return Some(version.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_content_hash() {
        let hash = compute_content_hash("test content");
        assert!(hash.starts_with("sha256:"));
        assert_eq!(hash.len(), 7 + 64); // "sha256:" + 64 hex chars
    }

    #[test]
    fn test_extract_version_from_frontmatter() {
        let content = r#"---
title: Test Document
version: "1.0.0"
---

# Content
"#;
        assert_eq!(
            extract_version_from_frontmatter(content),
            Some("1.0.0".to_string())
        );

        let no_version = r#"---
title: Test Document
---
"#;
        assert_eq!(extract_version_from_frontmatter(no_version), None);

        let no_frontmatter = "# Just content";
        assert_eq!(extract_version_from_frontmatter(no_frontmatter), None);
    }

    #[test]
    fn test_manifest_hash_determinism() {
        let artifacts = vec![
            GovernedArtifactRef {
                doc_id: "DOC-A".to_string(),
                path: "a.md".to_string(),
                content_hash: "sha256:aaa".to_string(),
                version: None,
                type_key: "test".to_string(),
            },
            GovernedArtifactRef {
                doc_id: "DOC-B".to_string(),
                path: "b.md".to_string(),
                content_hash: "sha256:bbb".to_string(),
                version: None,
                type_key: "test".to_string(),
            },
        ];

        let hash1 = compute_manifest_hash(&artifacts);
        let hash2 = compute_manifest_hash(&artifacts);

        assert_eq!(hash1, hash2);
        assert!(hash1.starts_with("sha256:"));
    }

    #[test]
    fn test_to_typed_refs() {
        let manifest = GovernedManifest {
            artifacts: vec![GovernedArtifactRef {
                doc_id: "SR-DIRECTIVE".to_string(),
                path: "charter/SR-DIRECTIVE.md".to_string(),
                content_hash: "sha256:abc123".to_string(),
                version: Some("1.0.0".to_string()),
                type_key: "governance.dev_directive".to_string(),
            }],
            computed_at: Utc::now(),
            manifest_hash: "sha256:manifest".to_string(),
        };

        let refs = manifest.to_typed_refs();
        assert_eq!(refs.len(), 1);

        let ref0 = &refs[0];
        assert_eq!(ref0.kind, "GovernedArtifact");
        assert_eq!(ref0.id, "SR-DIRECTIVE");
        assert_eq!(ref0.rel, "depends_on");
        assert_eq!(ref0.meta.get("content_hash").unwrap(), "sha256:abc123");
    }
}
