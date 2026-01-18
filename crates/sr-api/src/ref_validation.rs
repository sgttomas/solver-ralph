use serde_json::{Map, Value};
use sr_domain::refs::{validate_typed_refs, RefValidationError};
use sr_domain::TypedRef;

use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

/// Normalize refs with missing meta (where derivable) and validate per SR-SPEC §1.5.3
pub async fn normalize_and_validate_refs(
    state: &AppState,
    refs: Vec<TypedRef>,
) -> ApiResult<Vec<TypedRef>> {
    let mut normalized = Vec::with_capacity(refs.len());
    for r in refs {
        normalized.push(normalize_ref(state, r).await?);
    }

    validate_typed_refs(&normalized).map_err(ref_validation_error)?;

    Ok(normalized)
}

async fn normalize_ref(state: &AppState, mut r: TypedRef) -> ApiResult<TypedRef> {
    match r.kind.as_str() {
        "GovernedArtifact" => {
            let mut meta = ensure_meta_map(r.meta);
            if !meta.contains_key("content_hash") || !meta.contains_key("version") {
                if let Some(artifact) = state.governed_manifest.get_artifact(&r.id) {
                    meta.insert(
                        "content_hash".to_string(),
                        Value::String(artifact.content_hash.clone()),
                    );
                    if let Some(ver) = &artifact.version {
                        meta.insert("version".to_string(), Value::String(ver.clone()));
                    }
                    meta.insert(
                        "type_key".to_string(),
                        Value::String(artifact.type_key.clone()),
                    );
                }
            }
            r.meta = Value::Object(meta);
        }
        "Candidate" => {
            let mut meta = ensure_meta_map(r.meta);
            if !meta.contains_key("content_hash") {
                let candidate = state
                    .projections
                    .get_candidate(&r.id)
                    .await?
                    .ok_or_else(|| ApiError::NotFound {
                        resource: "Candidate".to_string(),
                        id: r.id.clone(),
                    })?;
                meta.insert(
                    "content_hash".to_string(),
                    Value::String(candidate.content_hash),
                );
            }
            r.meta = Value::Object(meta);
        }
        "OracleSuite" => {
            let mut meta = ensure_meta_map(r.meta);
            if !meta.contains_key("content_hash") {
                if let Some(suite) = state.oracle_registry.get_suite(&r.id).await {
                    meta.insert(
                        "content_hash".to_string(),
                        Value::String(suite.suite_hash.clone()),
                    );
                }
            }
            r.meta = Value::Object(meta);
        }
        "EvidenceBundle" => {
            let mut meta = ensure_meta_map(r.meta);
            if !meta.contains_key("content_hash") && r.id.starts_with("sha256:") {
                meta.insert("content_hash".to_string(), Value::String(r.id.clone()));
            }
            r.meta = Value::Object(meta);
        }
        "Record" => {
            let mut meta = ensure_meta_map(r.meta);
            if !meta.contains_key("content_hash") && r.id.starts_with("sha256:") {
                meta.insert("content_hash".to_string(), Value::String(r.id.clone()));
            }
            r.meta = Value::Object(meta);
        }
        _ => {}
    }

    Ok(r)
}

fn ensure_meta_map(meta: Value) -> Map<String, Value> {
    match meta {
        Value::Object(map) => map,
        _ => Map::new(),
    }
}

fn ref_validation_error(err: RefValidationError) -> ApiError {
    ApiError::BadRequest {
        message: format!("Invalid reference: {err}"),
    }
}
