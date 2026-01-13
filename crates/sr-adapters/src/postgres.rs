//! PostgreSQL adapter stubs
//!
//! Full implementation in D-10 (EventStore adapter)

use sr_ports::{EventStore, EventStoreError};
use sr_domain::EventEnvelope;

/// PostgreSQL-backed event store
///
/// Implements append-only event streams per SR-SPEC §1.6
pub struct PostgresEventStore {
    // Pool will be added in D-10
    _pool: (),
}

impl PostgresEventStore {
    /// Create a new PostgreSQL event store (stub)
    pub fn new(_database_url: &str) -> Self {
        Self { _pool: () }
    }
}

impl EventStore for PostgresEventStore {
    async fn append(
        &self,
        _stream_id: &str,
        _expected_version: u64,
        _events: Vec<EventEnvelope>,
    ) -> Result<u64, EventStoreError> {
        // Stub implementation - will be completed in D-10
        todo!("PostgresEventStore::append - implement in D-10")
    }

    async fn read_stream(
        &self,
        _stream_id: &str,
        _from_seq: u64,
        _limit: usize,
    ) -> Result<Vec<EventEnvelope>, EventStoreError> {
        // Stub implementation - will be completed in D-10
        todo!("PostgresEventStore::read_stream - implement in D-10")
    }

    async fn replay_all(
        &self,
        _from_global_seq: u64,
        _limit: usize,
    ) -> Result<Vec<EventEnvelope>, EventStoreError> {
        // Stub implementation - will be completed in D-10
        todo!("PostgresEventStore::replay_all - implement in D-10")
    }
}
