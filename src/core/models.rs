use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LinkState {
    OnlineNasaDirect,
    OfflineAirGappedBank,
    MarsDeepSpaceDelay,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlanetaryBody {
    Earth,
    Mars,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoCoordinate {
    pub body: PlanetaryBody,
    pub lat: f64,
    pub lon: f64,
    pub elevation_m: f64,
    pub climate_zone: String,
    pub grid_code: String,
    pub auto_located: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectralTelemetry {
    pub source_analyzed: bool,
    pub resolution: (u32, u32),
    pub mean_rgb: [f64; 3],
    pub vari_index: f64,
    pub pseudo_ndvi: f64,
    pub water_stress_index: f64,
    pub canopy_coverage_pct: f64,
    pub thermal_anomaly_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasaBankRecord {
    pub cell_key: String,
    pub body: String,
    pub lat: f64,
    pub lon: f64,
    pub source_dataset: String,
    pub soil_moisture_pct: f64,
    pub surface_temp_c: f64,
    pub solar_irradiance_kwh_m2: f64,
    pub precipitation_mm_day: f64,
    pub evapotranspiration_mm_day: f64,
    pub atmospheric_pressure_kpa: f64,
    pub ingested_at_utc: String,
    pub sha256_integrity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveForecast {
    pub horizon_days: u32,
    pub projected_soil_moisture_pct: f64,
    pub drought_risk_index: f64,
    pub recommended_action_window: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalWeatherAlert {
    pub alert_level: String, // "CRITICAL", "WARNING", "NOMINAL"
    pub hazard_type: String, // "HEATWAVE / THERMAL SHOCK", "DUST STORM / REGOLITH DRIFT", "FLASH FLOOD"
    pub trigger_condition: String,
    pub emergency_protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRequest {
    pub query: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub image_path: Option<String>,
    pub force_mars: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PillarResult {
    pub pillar_id: String,
    pub pillar_name: String,
    pub target_awards: Vec<String>,
    pub execution_time_us: u128,
    pub confidence_score: f64,
    pub diagnostic: String,
    pub metrics: HashMap<String, serde_json::Value>,
    pub recommended_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionStudyReport {
    pub report_id: String,
    pub generated_at_utc: String,
    pub link_state: LinkState,
    pub team: String,
    pub challenge: String,
    pub cartography: GeoCoordinate,
    pub photography: SpectralTelemetry,
    pub bank_record: NasaBankRecord,
    pub predictive_forecasts: Vec<PredictiveForecast>,
    pub weather_alert: CriticalWeatherAlert,
    pub pillars: Vec<PillarResult>,
    pub executive_synthesis: String,
    pub global_readiness_score: f64,
}
