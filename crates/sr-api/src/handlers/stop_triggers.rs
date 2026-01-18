//! Shared helpers for emitting StopTriggered events (C-LOOP-3)

use chrono::Utc;
use sr_domain::{ActorKind, EventEnvelope, EventId, StreamKind};
use sr_ports::EventStore;
use tracing::info;

use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

/// Emit a StopTriggered event and pause the loop.
///
/// `recommended_portal` can be provided to guide routing (SR-DIRECTIVE §4.2).
pub async fn emit_stop_triggered(
    state: &AppState,
    loop_id: &str,
    trigger: &str,
    requires_decision: bool,
    recommended_portal: Option<&str>,
) -> ApiResult<()> {
    // Read current stream version for optimistic concurrency
    let events = state.event_store.read_stream(loop_id, 0, 1000).await?;
    let current_version = events.len() as u64;

    let event_id = EventId::new();
    let now = Utc::now();

    let mut payload = serde_json::json!({
        "trigger": trigger,
        // Provide condition alias for listeners expecting `condition`
        "condition": trigger,
        "requires_decision": requires_decision,
    });

    if let Some(portal) = recommended_portal {
        payload["recommended_portal"] = serde_json::json!(portal);
    }

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: loop_id.to_string(),
        stream_kind: StreamKind::Loop,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "StopTriggered".to_string(),
        occurred_at: now,
        actor_kind: ActorKind::System,
        actor_id: "system:loop-governor".to_string(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state
        .event_store
        .append(loop_id, current_version, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    info!(
        loop_id = %loop_id,
        trigger = %trigger,
        requires_decision = %requires_decision,
        recommended_portal = ?recommended_portal,
        "StopTriggered event emitted - Loop paused"
    );

    Ok(())
}

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}
