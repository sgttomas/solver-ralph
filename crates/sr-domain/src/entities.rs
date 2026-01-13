//! Domain entities per SR-SPEC §1.2.3
//!
//! Platform domain types aligned with SR-TYPES §4.3

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

/// Actor kind per SR-SPEC §1.4.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActorKind {
    Human,
    Agent,
    System,
}

/// Actor identity per SR-SPEC §1.4.2
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorId {
    pub kind: ActorKind,
    pub id: String,
}

/// Loop identifier: `loop_<ULID>` per SR-SPEC §1.3.1
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoopId(String);

impl LoopId {
    pub fn new() -> Self {
        Self(format!("loop_{}", Ulid::new()))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for LoopId {
    fn default() -> Self {
        Self::new()
    }
}

/// Iteration identifier: `iter_<ULID>` per SR-SPEC §1.3.1
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IterationId(String);

impl IterationId {
    pub fn new() -> Self {
        Self(format!("iter_{}", Ulid::new()))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for IterationId {
    fn default() -> Self {
        Self::new()
    }
}

/// Candidate identifier per SR-SPEC §1.3.3
/// Format: `git:<commit_sha>|sha256:<manifest_hash>|cand_<ulid>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CandidateId(String);

impl CandidateId {
    pub fn new(git_sha: Option<&str>, content_hash: &str) -> Self {
        let ulid = Ulid::new();
        let mut parts = Vec::new();
        if let Some(sha) = git_sha {
            parts.push(format!("git:{sha}"));
        }
        parts.push(format!("sha256:{content_hash}"));
        parts.push(format!("cand_{ulid}"));
        Self(parts.join("|"))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Content hash per SR-SPEC §1.3.2
/// Format: `sha256:<64-hex>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentHash(String);

impl ContentHash {
    pub fn new(hex_digest: &str) -> Self {
        Self(format!("sha256:{hex_digest}"))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Loop state per SR-SPEC §3.1.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoopState {
    Created,
    Active,
    Paused,
    Closed,
}

/// Work Unit (Loop) entity per SR-TYPES `domain.work_unit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUnit {
    pub id: LoopId,
    pub goal: String,
    pub state: LoopState,
    pub created_at: DateTime<Utc>,
    pub created_by: ActorId,
}

/// Typed reference per SR-SPEC §1.5.3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedRef {
    pub kind: String,
    pub id: String,
    pub rel: String,
    #[serde(default)]
    pub meta: serde_json::Value,
}
