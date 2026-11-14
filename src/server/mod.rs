use crate::core::models::{AgentRequest, MissionStudyReport, NasaBankRecord};
use crate::core::orchestrator::AeroCropNanoAgent;
use crate::export::markdown::export_study_report;
use crate::pipeline::daemon::SovereignDaemon;
use crate::pipeline::databank::NasaDataBank;
use axum::{
    extract::Json,
    response::Html,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
pub struct ApiAuditPayload {
    pub query: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub image_path: Option<String>,
    pub force_mars: Option<bool>,
}

async fn cockpit_ui() -> Html<String> {
    let html = std::fs::read_to_string("web/index.html")
        .unwrap_or_else(|_| "<h1>Cockpit Web 3D introuvable dans web/index.html</h1>".into());
    Html(html)
}

async fn health() -> Json<serde_json::Value> {
    let bank = NasaDataBank::open();
    Json(serde_json::json!({
        "status": "ONLINE",
        "engine": "100% Pure Rust Deterministic Bio-Spatial Kernel",
        "databank_cells": bank.count_records()
    }))
}

async fn list_bank() -> Json<Vec<NasaBankRecord>> {
    let bank = NasaDataBank::open();
    Json(bank.list_all())
}

async fn run_audit(Json(payload): Json<ApiAuditPayload>) -> Json<MissionStudyReport> {
    let req = AgentRequest {
        query: payload.query,
        lat: payload.lat,
        lon: payload.lon,
        image_path: payload.image_path,
        force_mars: payload.force_mars.unwrap_or(false),
    };
    let report = AeroCropNanoAgent::execute(req).await;
    let _ = export_study_report(&report);
    Json(report)
}

pub async fn start_server(port: u16) {
    // Lancement du Démon Autonome en tâche de fond parallèle
    tokio::spawn(async {
        SovereignDaemon::run_background_loop(300).await;
    });

    let app = Router::new()
        .route("/", get(cockpit_ui))
        .route("/health", get(health))
        .route("/api/v1/bank", get(list_bank))
        .route("/api/v1/audit", post(run_audit))
        .layer(CorsLayer::permissive());

    let addr = format!("0.0.0.0:{}", port);
    println!("[EVHT-COCKPIT] Cockpit Web 3D & API Souveraine actifs sur http://localhost:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
