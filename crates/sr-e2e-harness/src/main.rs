//! SOLVER-Ralph E2E Harness CLI (D-34, D-36)
//!
//! Command-line tool for running the end-to-end harness and replay verification.
//!
//! Usage:
//!   sr-e2e-harness [OPTIONS]
//!
//! Commands:
//!   (default)           Run E2E happy path flow
//!   --replay            Replay event stream and verify determinism
//!   --verify-determinism  Run replay twice and verify identical checksums
//!
//! Options:
//!   --api-url URL       API base URL (default: http://localhost:3000)
//!   --system-token TOK  SYSTEM actor auth token
//!   --human-token TOK   HUMAN actor auth token
//!   --database-url URL  Database connection URL (for replay)
//!   --output FILE       Output transcript to file (default: stdout)
//!   --json              Output JSON transcript
//!   --help              Show help

use sr_e2e_harness::{run_happy_path, HarnessConfig, ReplayConfig, ReplayRunner};
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

    // Check for replay mode
    if args.iter().any(|a| a == "--replay") {
        run_replay_mode(&args).await;
        return;
    }

    // Check for determinism verification mode
    if args.iter().any(|a| a == "--verify-determinism") {
        run_determinism_mode(&args).await;
        return;
    }

    // Default: run E2E happy path
    run_e2e_mode(&args).await;
}

/// Run E2E happy path mode
async fn run_e2e_mode(args: &[String]) {
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
                // Ignore unknown arguments silently for forward compatibility
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

/// Run replay mode - rebuild projections from event stream
async fn run_replay_mode(args: &[String]) {
    let mut replay_config = ReplayConfig::default();
    let mut output_file: Option<String> = None;
    let mut json_output = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--database-url" => {
                if i + 1 < args.len() {
                    replay_config.database_url = args[i + 1].clone();
                    i += 1;
                }
            }
            "--batch-size" => {
                if i + 1 < args.len() {
                    replay_config.batch_size = args[i + 1].parse().unwrap_or(100);
                    i += 1;
                }
            }
            "--incremental" => {
                replay_config.full_rebuild = false;
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
            _ => {}
        }
        i += 1;
    }

    println!("==============================================");
    println!("  SOLVER-Ralph Replay Tool (D-36)");
    println!("==============================================");
    println!();
    println!("Mode: {}", if replay_config.full_rebuild { "Full Rebuild" } else { "Incremental" });
    println!("Batch Size: {}", replay_config.batch_size);
    println!();

    // Create replay runner
    let runner = match ReplayRunner::new(replay_config).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to create replay runner: {}", e);
            std::process::exit(1);
        }
    };

    // Run replay
    let result = match runner.run().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Replay failed: {}", e);
            std::process::exit(1);
        }
    };

    // Output transcript
    if json_output || output_file.is_some() {
        let transcript_json = result
            .transcript
            .to_json()
            .expect("Failed to serialize transcript");

        if let Some(ref file_path) = output_file {
            let mut file = File::create(file_path).expect("Failed to create output file");
            file.write_all(transcript_json.as_bytes())
                .expect("Failed to write transcript");
            println!("Transcript written to: {}", file_path);
        }

        if json_output {
            println!("\n=== REPLAY TRANSCRIPT ===\n");
            println!("{}", transcript_json);
        }
    }

    // Print summary
    println!("\n==============================================");
    println!("  REPLAY RESULT");
    println!("==============================================");
    println!();
    println!("Transcript ID: {}", result.transcript.transcript_id);
    println!("Events Processed: {}", result.events_processed);
    println!("Determinism Verified: {}", result.determinism_verified);
    println!();
    println!("State Checksum: {}", result.state_checksum.checksum);
    println!("Event Count: {}", result.state_checksum.event_count);
    println!("Last Global Seq: {}", result.state_checksum.last_global_seq);
    println!();

    // Print component checksums
    println!("Component Checksums:");
    for (table, hash) in &result.state_checksum.component_hashes {
        println!("  {}: {} rows, hash={}", table, hash.row_count, hash.content_hash);
    }
    println!();

    println!("Replay completed successfully!");
}

/// Run determinism verification mode - replay twice and compare checksums
async fn run_determinism_mode(args: &[String]) {
    let mut replay_config = ReplayConfig::default();
    let mut output_file: Option<String> = None;
    let mut json_output = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--database-url" => {
                if i + 1 < args.len() {
                    replay_config.database_url = args[i + 1].clone();
                    i += 1;
                }
            }
            "--batch-size" => {
                if i + 1 < args.len() {
                    replay_config.batch_size = args[i + 1].parse().unwrap_or(100);
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
            _ => {}
        }
        i += 1;
    }

    println!("==============================================");
    println!("  SOLVER-Ralph Determinism Verification (D-36)");
    println!("==============================================");
    println!();
    println!("This will replay the event stream twice and");
    println!("verify that both replays produce identical state.");
    println!();

    // Create replay runner
    let runner = match ReplayRunner::new(replay_config).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to create replay runner: {}", e);
            std::process::exit(1);
        }
    };

    // Run determinism verification
    let result = match runner.verify_determinism().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Determinism verification failed: {}", e);
            std::process::exit(1);
        }
    };

    // Output result
    if json_output || output_file.is_some() {
        let result_json = serde_json::to_string_pretty(&result)
            .expect("Failed to serialize result");

        if let Some(ref file_path) = output_file {
            let mut file = File::create(file_path).expect("Failed to create output file");
            file.write_all(result_json.as_bytes())
                .expect("Failed to write result");
            println!("Result written to: {}", file_path);
        }

        if json_output {
            println!("\n=== DETERMINISM RESULT ===\n");
            println!("{}", result_json);
        }
    }

    // Print summary
    println!("\n==============================================");
    println!("  DETERMINISM VERIFICATION RESULT");
    println!("==============================================");
    println!();
    println!("First Replay:");
    println!("  Events Processed: {}", result.first_events_processed);
    println!("  State Checksum: {}", result.first_checksum);
    println!();
    println!("Second Replay:");
    println!("  Events Processed: {}", result.second_events_processed);
    println!("  State Checksum: {}", result.second_checksum);
    println!();

    if result.checksums_match {
        println!("RESULT: DETERMINISM VERIFIED");
        println!("  Both replays produced identical state checksums.");
        println!();
        println!("Determinism verification completed successfully!");
    } else {
        eprintln!("RESULT: DETERMINISM FAILURE");
        eprintln!("  Replays produced different state checksums!");
        eprintln!("  This indicates non-deterministic projection behavior.");
        std::process::exit(1);
    }
}

fn print_help() {
    println!(
        r#"SOLVER-Ralph E2E Harness (D-34, D-36)

Usage:
  sr-e2e-harness [OPTIONS]
  sr-e2e-harness --replay [OPTIONS]
  sr-e2e-harness --verify-determinism [OPTIONS]

Commands:
  (default)              Run E2E happy path flow
  --replay               Replay event stream and rebuild projections
  --verify-determinism   Replay twice and verify identical checksums

E2E Options:
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

Replay Options:
  --database-url URL      Database connection URL
                          Can also be set via DATABASE_URL env var
  --batch-size N          Event processing batch size (default: 100)
  --incremental           Incremental update instead of full rebuild

Common Options:
  --output FILE           Write transcript/result to file
  --json                  Print JSON output to stdout
  --help                  Show this help

Environment Variables:
  SR_API_URL              API base URL
  SR_SYSTEM_TOKEN         SYSTEM actor auth token
  SR_HUMAN_TOKEN          HUMAN actor auth token
  DATABASE_URL            Database connection URL
  RUST_LOG                Log level (default: info)

Examples:
  # Run E2E happy path against local API
  sr-e2e-harness

  # Run against custom API with tokens
  sr-e2e-harness --api-url https://api.example.com \
    --system-token sys123 --human-token human456

  # Output E2E transcript to file
  sr-e2e-harness --output transcript.json --json

  # Replay event stream and verify state
  sr-e2e-harness --replay --database-url postgres://user:pass@localhost/db

  # Verify determinism (replay twice, compare checksums)
  sr-e2e-harness --verify-determinism --json

E2E Description (D-34):
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

Replay Description (D-36):
  Per SR-SPEC 1.7, all projections must be rebuildable from es.events alone.
  The replay tool:
  - Truncates projection tables (full rebuild mode)
  - Replays all events from the event store
  - Recomputes all projection state
  - Produces a deterministic state checksum

  The determinism verification mode runs replay twice and verifies that
  both runs produce identical state checksums, proving that projection
  rebuild is deterministic.
"#
    );
}
