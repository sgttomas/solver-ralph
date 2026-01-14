//! SOLVER-Ralph E2E Harness CLI (D-34)
//!
//! Command-line tool for running the end-to-end harness.
//!
//! Usage:
//!   sr-e2e-harness [OPTIONS]
//!
//! Options:
//!   --api-url URL       API base URL (default: http://localhost:3000)
//!   --system-token TOK  SYSTEM actor auth token
//!   --human-token TOK   HUMAN actor auth token
//!   --output FILE       Output transcript to file (default: stdout)
//!   --json              Output JSON transcript
//!   --help              Show help

use sr_e2e_harness::{run_happy_path, HarnessConfig};
use std::env;
use std::fs::File;
use std::io::Write;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Parse arguments
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    // Build config from args and env
    let mut config = HarnessConfig::default();
    let mut output_file: Option<String> = None;
    let mut json_output = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--api-url" => {
                if i + 1 < args.len() {
                    config.api_base_url = args[i + 1].clone();
                    i += 1;
                }
            }
            "--system-token" => {
                if i + 1 < args.len() {
                    config.system_token = args[i + 1].clone();
                    i += 1;
                }
            }
            "--human-token" => {
                if i + 1 < args.len() {
                    config.human_token = args[i + 1].clone();
                    i += 1;
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--json" => {
                json_output = true;
            }
            "--baseline-id" => {
                if i + 1 < args.len() {
                    config.baseline_id = args[i + 1].clone();
                    i += 1;
                }
            }
            "--oracle-suite-id" => {
                if i + 1 < args.len() {
                    config.oracle_suite_id = args[i + 1].clone();
                    i += 1;
                }
            }
            "--oracle-suite-hash" => {
                if i + 1 < args.len() {
                    config.oracle_suite_hash = args[i + 1].clone();
                    i += 1;
                }
            }
            "--no-verify-evidence" => {
                config.verify_evidence = false;
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
            }
        }
        i += 1;
    }

    println!("==============================================");
    println!("  SOLVER-Ralph E2E Harness (D-34)");
    println!("==============================================");
    println!();
    println!("API URL: {}", config.api_base_url);
    println!("Oracle Suite: {}", config.oracle_suite_id);
    println!("Baseline ID: {}", config.baseline_id);
    println!();

    // Run the harness
    let result = run_happy_path(config).await;

    // Output transcript
    if json_output || output_file.is_some() {
        let transcript_json = result
            .transcript
            .to_deterministic_json()
            .expect("Failed to serialize transcript");

        if let Some(ref file_path) = output_file {
            let mut file = File::create(file_path).expect("Failed to create output file");
            file.write_all(transcript_json.as_bytes())
                .expect("Failed to write transcript");
            println!("Transcript written to: {}", file_path);
        }

        if json_output {
            println!("\n=== TRANSCRIPT ===\n");
            println!("{}", transcript_json);
        }
    }

    // Print summary
    println!("\n==============================================");
    println!("  HARNESS RESULT");
    println!("==============================================");
    println!();
    println!(
        "Status: {}",
        if result.success { "SUCCESS" } else { "FAILED" }
    );
    println!("Transcript ID: {}", result.transcript.transcript_id);
    println!(
        "Content Hash: {}",
        result.transcript.content_hash.as_deref().unwrap_or("N/A")
    );
    println!();

    // Print produced IDs
    println!("Produced Entities:");
    if let Some(ref loop_id) = result.transcript.produced_ids.loop_id {
        println!("  Loop: {}", loop_id);
    }
    for iter_id in &result.transcript.produced_ids.iteration_ids {
        println!("  Iteration: {}", iter_id);
    }
    for cand_id in &result.transcript.produced_ids.candidate_ids {
        println!("  Candidate: {}", cand_id);
    }
    for run_id in &result.transcript.produced_ids.run_ids {
        println!("  Run: {}", run_id);
    }
    for evidence_hash in &result.transcript.produced_ids.evidence_hashes {
        println!("  Evidence: {}", evidence_hash);
    }
    for approval_id in &result.transcript.produced_ids.approval_ids {
        println!("  Approval: {}", approval_id);
    }
    for freeze_id in &result.transcript.produced_ids.freeze_ids {
        println!("  Freeze: {}", freeze_id);
    }
    println!();

    // Print invariant checks
    println!("Invariant Checks:");
    for check in &result.transcript.invariants_checked {
        let status = if check.passed { "[PASS]" } else { "[FAIL]" };
        println!("  {} {}: {}", status, check.name, check.message);
    }
    println!();

    // Print events count
    println!(
        "Total Events: {}",
        result.transcript.produced_ids.event_ids.len()
    );
    println!();

    if let Some(ref error) = result.error {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }

    if !result.transcript.all_invariants_passed() {
        eprintln!("Some invariants failed!");
        std::process::exit(1);
    }

    println!("E2E harness completed successfully!");
}

fn print_help() {
    println!(
        r#"SOLVER-Ralph E2E Harness (D-34)

Usage:
  sr-e2e-harness [OPTIONS]

Options:
  --api-url URL           API base URL (default: http://localhost:3000)
                          Can also be set via SR_API_URL env var
  --system-token TOKEN    SYSTEM actor auth token
                          Can also be set via SR_SYSTEM_TOKEN env var
  --human-token TOKEN     HUMAN actor auth token
                          Can also be set via SR_HUMAN_TOKEN env var
  --oracle-suite-id ID    Oracle suite ID (default: sr-core-suite)
  --oracle-suite-hash H   Oracle suite hash (default: 0x00...64)
  --baseline-id ID        Baseline ID for freeze record
  --no-verify-evidence    Skip evidence verification step
  --output FILE           Write transcript to file
  --json                  Print JSON transcript to stdout
  --help                  Show this help

Environment Variables:
  SR_API_URL              API base URL
  SR_SYSTEM_TOKEN         SYSTEM actor auth token
  SR_HUMAN_TOKEN          HUMAN actor auth token
  RUST_LOG                Log level (default: info)

Examples:
  # Run against local API
  sr-e2e-harness

  # Run against custom API with tokens
  sr-e2e-harness --api-url https://api.example.com \
    --system-token sys123 --human-token human456

  # Output transcript to file
  sr-e2e-harness --output transcript.json --json

Description:
  This harness executes a representative end-to-end loop:
  1. Create and activate a loop
  2. Start an iteration (SYSTEM actor)
  3. Register a candidate (worker submission)
  4. Run oracles and upload evidence
  5. Record approval (HUMAN at Release Portal)
  6. Create freeze record (establishing baseline)
  7. Close the loop

  The harness asserts key invariants:
  - No approvals without evidence
  - HUMAN-only for approvals and freeze
  - SYSTEM-only for iteration start
  - Evidence integrity verification

  The transcript provides a deterministic audit trail of all operations.
"#
    );
}
