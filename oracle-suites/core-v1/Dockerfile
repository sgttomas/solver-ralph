# SOLVER-Ralph Core Oracle Suite v1
#
# Container image for executing core verification oracles:
# - oracle:build - Verify project builds successfully
# - oracle:unit-tests - Run unit test suite
# - oracle:schema-validation - Validate JSON schemas
# - oracle:lint - Run code linting (advisory)
#
# Per SR-SPEC §4.5 (Oracle runtime pattern):
# - OCI container image pinned by digest
# - Runtime: gVisor (runsc) for sandboxing
# - Network disabled by default
# - Read-only workspace mount at /workspace
# - Write-only scratch volume for outputs at /scratch

FROM rust:1.83-bookworm

# Install additional dependencies for multi-language support
RUN apt-get update && apt-get install -y --no-install-recommends \
    nodejs \
    npm \
    python3 \
    python3-pip \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Install Rust components for linting
RUN rustup component add clippy rustfmt

# Create oracle scripts directory
RUN mkdir -p /oracles /scratch/reports

# Copy oracle scripts
COPY oracles/ /oracles/

# Make scripts executable
RUN chmod +x /oracles/*.sh

# Set working directory to workspace (read-only mount point)
WORKDIR /workspace

# Environment variables for oracle scripts
ENV WORKSPACE_MOUNT=/workspace
ENV SCRATCH_MOUNT=/scratch
ENV REPORTS_DIR=/scratch/reports

# Default entrypoint (overridden by oracle runner)
ENTRYPOINT ["/bin/bash"]
