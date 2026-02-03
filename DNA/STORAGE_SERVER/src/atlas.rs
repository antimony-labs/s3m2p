//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: atlas.rs | DNA/STORAGE_SERVER/src/atlas.rs
//! PURPOSE: ATLAS map data endpoints
//! MODIFIED: 2026-01-25
//! ═══════════════════════════════════════════════════════════════════════════════

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tokio::fs;
use tracing::info;

use crate::AppState;

/// Get map layer data
/// GET /v1/atlas/:layer/:resolution
/// e.g., /v1/atlas/countries/110m -> countries_110m.geojson
pub async fn get_layer(
    State(state): State<AppState>,
    Path((layer, resolution)): Path<(String, String)>,
) -> Response {
    // Validate layer name (prevent path traversal)
    let valid_layers = [
        "countries",
        "states",
        "places",
        "rivers",
        "lakes",
        "coastline",
    ];
    let valid_resolutions = ["110m", "50m", "10m"];

    if !valid_layers.contains(&layer.as_str()) {
        return (StatusCode::BAD_REQUEST, "Invalid layer").into_response();
    }
    if !valid_resolutions.contains(&resolution.as_str()) {
        return (StatusCode::BAD_REQUEST, "Invalid resolution").into_response();
    }

    // Construct path: data/atlas/{layer}_{resolution}.geojson
    let filename = format!("{}_{}.geojson", layer, resolution);
    let mut path = state.data_dir.clone();
    path.push("atlas");
    path.push(&filename);

    info!("Serving ATLAS layer: {}", path.display());

    match fs::read(&path).await {
        Ok(data) => {
            let headers = [
                ("Content-Type", "application/geo+json"),
                ("Cache-Control", "public, max-age=86400"), // 24h cache
            ];
            (StatusCode::OK, headers, data).into_response()
        }
        Err(_) => {
            // Try .geo binary format
            let binary_filename = format!("{}_{}.geo", layer, resolution);
            let mut binary_path = state.data_dir.clone();
            binary_path.push("atlas");
            binary_path.push(&binary_filename);

            match fs::read(&binary_path).await {
                Ok(data) => {
                    let headers = [
                        ("Content-Type", "application/octet-stream"),
                        ("Cache-Control", "public, max-age=86400"),
                    ];
                    (StatusCode::OK, headers, data).into_response()
                }
                Err(_) => StatusCode::NOT_FOUND.into_response(),
            }
        }
    }
}

/// List available layers
/// GET /v1/atlas/layers
pub async fn list_layers(State(state): State<AppState>) -> Response {
    let mut path = state.data_dir.clone();
    path.push("atlas");

    let mut layers = Vec::new();

    if let Ok(mut entries) = fs::read_dir(&path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(name) = entry.file_name().into_string() {
                if name.ends_with(".geojson") || name.ends_with(".geo") {
                    layers.push(name);
                }
            }
        }
    }

    layers.sort();
    let json = serde_json::to_string(&layers).unwrap_or_else(|_| "[]".to_string());

    let headers = [("Content-Type", "application/json")];
    (StatusCode::OK, headers, json).into_response()
}

/// Health check for ATLAS endpoint
pub async fn atlas_health() -> impl IntoResponse {
    (StatusCode::OK, "ATLAS OK")
}
