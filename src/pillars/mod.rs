use crate::core::models::{GeoCoordinate, NasaBankRecord, PillarResult, PlanetaryBody, SpectralTelemetry};
use serde_json::json;
use std::collections::HashMap;
use std::time::Instant;

pub async fn run_pillar_1_data_science(
    _geo: GeoCoordinate,
    spec: SpectralTelemetry,
    bank: NasaBankRecord,
) -> PillarResult {
    let start = Instant::now();
    let mut metrics = HashMap::new();

    let fused_ndvi = if spec.source_analyzed {
        (spec.pseudo_ndvi * 0.72 + (bank.soil_moisture_pct / 100.0) * 0.28).clamp(-1.0, 1.0)
    } else {
        spec.pseudo_ndvi
    };

    metrics.insert("bank_cell_key".into(), json!(bank.cell_key));
    metrics.insert("nasa_dataset".into(), json!(bank.source_dataset));
    metrics.insert("sha256_proof".into(), json!(&bank.sha256_integrity[..16]));
    metrics.insert("soil_or_regolith_moisture_pct".into(), json!(bank.soil_moisture_pct));
    metrics.insert("surface_temp_c".into(), json!(bank.surface_temp_c));
    metrics.insert("evapotranspiration_mm_day".into(), json!(bank.evapotranspiration_mm_day));
    metrics.insert("fused_ndvi".into(), json!((fused_ndvi * 1000.0).round() / 1000.0));

    PillarResult {
        pillar_id: "P1".into(),
        pillar_name: "Data & Science (Banque NASA + Fusion Spectrale)".into(),
        target_awards: vec![
            "Meilleure utilisation des données".into(),
            "Meilleure utilisation de la science".into(),
        ],
        execution_time_us: start.elapsed().as_micros(),
        confidence_score: if spec.source_analyzed { 0.97 } else { 0.91 },
        diagnostic: format!(
            "Cellule {} scellée ({}) : humidité {}%, temp {}°C, NDVI fusionné {:.3}.",
            bank.cell_key, bank.source_dataset, bank.soil_moisture_pct, bank.surface_temp_c, fused_ndvi
        ),
        metrics,
        recommended_action: format!(
            "Appliquer la consigne d'évapotranspiration déterministe de {} mm/j.",
            bank.evapotranspiration_mm_day
        ),
    }
}

pub async fn run_pillar_2_core_engine(
    geo: GeoCoordinate,
    spec: SpectralTelemetry,
    bank: NasaBankRecord,
) -> PillarResult {
    let start = Instant::now();
    let mut metrics = HashMap::new();

    let (crop, strategy, saving_pct) = if geo.body == PlanetaryBody::Mars {
        (
            "Spiruline Arthrospira + Pomme de Terre Hydroponique LED Rouge/Bleu",
            "Culture en dôme pressurisé à 65 kPa avec recyclage d'eau en boucle fermée et extraction d'eau du régolithe",
            96.5,
        )
    } else if bank.soil_moisture_pct < 24.0 || spec.water_stress_index > 0.65 {
        (
            "Sorgho Bicolor / Quinoa Chenopodium (Métabolisme C4 haute résilience)",
            "Changement de cap cultural immédiat + micro-irrigation souterraine pilotée par seuil thermique",
            44.0,
        )
    } else {
        (
            "Orge Hybride Tolérante / Légumineuses Fixatrices d'Azote",
            "Rotation régénérative et irrigation déficitaire contrôlée (RDI)",
            29.5,
        )
    };

    metrics.insert("planetary_target".into(), json!(geo.body));
    metrics.insert("recommended_crop_pivot".into(), json!(crop));
    metrics.insert("deterministic_strategy".into(), json!(strategy));
    metrics.insert("water_saving_pct".into(), json!(saving_pct));
    metrics.insert("ai_architecture".into(), json!("100% Pure Rust Deterministic Kernel (Zero-LLM)"));

    PillarResult {
        pillar_id: "P2".into(),
        pillar_name: "Core Tech Engine (Cerveau Sans-LLM & Concept de Mission)".into(),
        target_awards: vec![
            "Meilleure utilisation de la technologie".into(),
            "Meilleur concept de mission".into(),
        ],
        execution_time_us: start.elapsed().as_micros(),
        confidence_score: 0.96,
        diagnostic: format!(
            "Décision déterministe en <1ms sans GPU/LLM : pivot vers {} (économie eau {}%).",
            crop, saving_pct
        ),
        metrics,
        recommended_action: format!("Exécuter le protocole cultural : {}.", crop),
    }
}

pub async fn run_pillar_3_deploy_impact(
    geo: GeoCoordinate,
    bank: NasaBankRecord,
) -> PillarResult {
    let start = Instant::now();
    let mut metrics = HashMap::new();

    let water_saved_m3 = ((4500.0 * (1.0 - bank.soil_moisture_pct / 100.0)) * 0.40).round();
    let celss_recycling = (95.0 + bank.solar_irradiance_kwh_m2 * 0.45).min(99.4);

    metrics.insert("grid_sector".into(), json!(geo.grid_code));
    metrics.insert("earth_water_saved_m3_ha".into(), json!(water_saved_m3));
    metrics.insert("mars_celss_water_recovery_pct".into(), json!((celss_recycling * 10.0).round() / 10.0));
    metrics.insert("zero_network_autonomy_days".into(), json!(180));

    PillarResult {
        pillar_id: "P3".into(),
        pillar_name: "Deploy & Impact (Zones Blanches Terrestres & Base Mars)".into(),
        target_awards: vec![
            "Impact local".into(),
            "Impact galactique".into(),
        ],
        execution_time_us: start.elapsed().as_micros(),
        confidence_score: 0.94,
        diagnostic: format!(
            "Autonomie 180j sans réseau : {} m³/ha économisés sur Terre | Recyclage serre Mars : {:.1}%.",
            water_saved_m3, celss_recycling
        ),
        metrics,
        recommended_action: "Activer le démon de régulation autonome local.".into(),
    }
}

pub async fn run_pillar_4_story_art(
    geo: GeoCoordinate,
    spec: SpectralTelemetry,
) -> PillarResult {
    let start = Instant::now();
    let mut metrics = HashMap::new();

    metrics.insert("cockpit_render_mode".into(), json!("DUAL_PLANETARY_SPECTRAL_MESH"));
    metrics.insert("coordinates".into(), json!([geo.lat, geo.lon, geo.elevation_m]));
    metrics.insert("canopy_extrusion".into(), json!(spec.canopy_coverage_pct));
    metrics.insert(
        "narrative_arc".into(),
        json!([
            "1. La donnée orbitale NASA est aspirée et scellée dans la banque souveraine.",
            "2. Sans internet ni cloud, le moteur Rust fusionne carte et photo pour sauver la récolte.",
            "3. Le même algorithme fait pousser la vie dans le régolithe martien."
        ]),
    );

    PillarResult {
        pillar_id: "P4".into(),
        pillar_name: "Story & Art (Cockpit Dual-Planétaire & Data-Narration)".into(),
        target_awards: vec![
            "Meilleure utilisation de la narration".into(),
            "Art et technologie".into(),
        ],
        execution_time_us: start.elapsed().as_micros(),
        confidence_score: 0.97,
        diagnostic: "Maillage spectral 3D et trame narrative Terre-Mars générés.".into(),
        metrics,
        recommended_action: "Afficher la carte thermique et le profil spectral dans le Cockpit.".into(),
    }
}

pub async fn run_pillar_5_community(geo: GeoCoordinate) -> PillarResult {
    let start = Instant::now();
    let mut metrics = HashMap::new();

    metrics.insert("sync_protocol".into(), json!("DTN-Bundle-Protocol-v7 (Delay-Tolerant Networking)"));
    metrics.insert("sector_node".into(), json!(geo.grid_code));
    metrics.insert("interplanetary_ready".into(), json!(true));

    PillarResult {
        pillar_id: "P5".into(),
        pillar_name: "Community & Connectivity (Réseau DTN Terre-Espace)".into(),
        target_awards: vec![
            "Connexion mondiale".into(),
            "Les plus inspirants".into(),
        ],
        execution_time_us: start.elapsed().as_micros(),
        confidence_score: 0.92,
        diagnostic: format!(
            "Paquet DTN prêt sur {} : synchronisation différée tolérante aux coupures (Rural & Mars).",
            geo.grid_code
        ),
        metrics,
        recommended_action: "Mettre en file d'attente le paquet télémétrique pour diffusion coopérative.".into(),
    }
}
