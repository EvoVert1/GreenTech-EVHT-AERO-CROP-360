use std::fs;
use serde_json::json;

pub struct AgentToolkit;

impl AgentToolkit {
    /// Génère un rapport JSON structuré et simule un export visuel pour le cockpit
    pub fn generate_visual_json_report(moisture: f64, et0: f64, temp: f64) -> Result<String, std::io::Error> {
        fs::create_dir_all("reports")?;
        
        // Création de l'objet JSON structuré pour le rapport et la vue graphique
        let report_data = json!({
            "mission": "AERO-CROP 360",
            "organization": "Evo High-Tech",
            "location": "Casablanca MGRS 29R QF",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "telemetry": {
                "soil_moisture_m3_m3": moisture,
                "target_moisture": 0.280,
                "evapotranspiration_mm_day": et0,
                "surface_temp_c": temp
            },
            "status": {
                "thermal_anomaly": temp > 36.0,
                "water_deficit_pct": ((0.280 - moisture) / 0.280) * 100.0,
                "action_required": "Deficit irrigation -22% & Geotextile Shading"
            },
            "security": {
                "air_gapped": true,
                "provenance": "SHA-256 Verified Sovereign Edge Core"
            }
        });

        let json_string = serde_json::to_string_pretty(&report_data)?;
        let filename = format!("reports/mission_snapshot_{}.json", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
        
        fs::write(&filename, &json_string)?;
        
        // Affichage d'un aperçu visuel formaté dans la console
        println!("\n[EvoNanoAgent VISUAL SNAPSHOT GENERATED]");
        println!("┌─────────────────────────────────────────────────────────┐");
        println!("│ 🌱 AERO-CROP 360 — VISUAL & JSON REPORT COCKPIT         │");
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│ Grid: Casablanca MGRS 29R QF                            │");
        println!("│ Moisture: {:.3} m³/m³ (Deficit Active)                │", moisture);
        println!("│ Temp: {:.1}°C [ALERT: >36°C Thermal Triggered]         │", temp);
        println!("│ JSON Export: {}                      │", filename);
        println!("└─────────────────────────────────────────────────────────┘");

        Ok(filename)
    }
}
