# SOLVER-Ralph Semantic Oracle Suite v1
#
# Container image for executing semantic verification oracles:
# - oracle:semantic-eval - Evaluate semantic compliance against meaning matrices
#
# Per SR-SEMANTIC-ORACLE-SPEC:
# - Semantic set definitions bundled in container (content-hashed)
# - Produces residual, coverage, violations reports
# - Outputs sr.semantic_eval.v1 schema
#
# Environment:
# - OCI container image pinned by digest
# - Runtime: gVisor (runsc) for sandboxing
# - Network disabled by default
# - Read-only workspace mount at /workspace
# - Write-only scratch volume for outputs at /scratch

FROM rust:1.83-bookworm AS builder

WORKDIR /build

# Copy workspace files needed for build
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build the oracles binary (release mode)
RUN cargo build --release --package sr-oracles

# Runtime image (slim)
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Create directories
RUN mkdir -p /oracles /scratch/reports/semantic /semantic-sets

# Copy the oracles binary
COPY --from=builder /build/target/release/sr-oracles /usr/local/bin/

# Copy oracle scripts
COPY oracle-suites/semantic-v1/oracles/ /oracles/

# Copy bundled semantic set definition (content-hashed)
COPY oracle-suites/semantic-v1/semantic-sets/ /semantic-sets/

# Make scripts executable
RUN chmod +x /oracles/*.sh

# Set working directory to workspace (read-only mount point)
WORKDIR /workspace

# Environment variables for oracle scripts
ENV WORKSPACE_MOUNT=/workspace
ENV SCRATCH_MOUNT=/scratch
ENV REPORTS_DIR=/scratch/reports
ENV SEMANTIC_SETS_DIR=/semantic-sets

# Default entrypoint (overridden by oracle runner)
ENTRYPOINT ["/bin/bash"]
