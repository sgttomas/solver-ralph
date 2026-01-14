//! SOLVER-Ralph End-to-End Harness (D-34)
//!
//! Automated harness to execute a representative end-to-end loop:
//! 1. Create loop → Start iteration → Worker submits candidate
//! 2. Oracles run → Evidence stored → Approval recorded → Freeze baseline
//!
//! Per SR-PLAN D-34:
//! - Harness can run in self-host mode and produce deterministic event+evidence transcript
//! - Harness asserts key invariants (no approvals without evidence, etc.)

pub mod client;
pub mod harness;
pub mod transcript;

pub use client::E2EClient;
pub use harness::{run_happy_path, HarnessConfig, HarnessError, HarnessResult};
pub use transcript::{HarnessTranscript, TranscriptEntry, TranscriptEntryKind};
