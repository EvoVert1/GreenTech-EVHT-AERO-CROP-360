use crate::core::models::MissionStudyReport;
use std::fs;

pub fn export_study_report(report: &MissionStudyReport) -> std::io::Result<String> {
    let mut md = String::new();
    md.push_str(&format!("# DOSSIER DE CANDIDATURE OFFICIEL — NASA SPACE APPS CHALLENGE\n"));
    md.push_str(&format!("## Projet : AERO-CROP 360 (Evo High-Tech)\n\n"));
    md.push_str(&format!("- **ID Rapport :** `{}`\n", report.report_id));
    md.push_str(&format!("- **Équipe :** {}\n", report.team));
    md.push_str(&format!("- **Défi Ciblé :** {}\n", report.challenge));
    md.push_str(&format!("- **État de Liaison :** `{:?}`\n", report.link_state));
    md.push_str(&format!("- **Preuve Cryptographique Banque NASA (SHA-256) :** `{}`\n", report.bank_record.sha256_integrity));
    md.push_str(&format!("- **Score Global de Préparation Mission :** `{}%`\n\n", report.global_readiness_score));

    md.push_str("--- \n\n## 1. SYNTHÈSE EXÉCUTIVE & IMPACT\n");
    md.push_str(&format!("> {}\n\n", report.executive_synthesis));

    md.push_str("## 2. MODULE D'ALERTE MÉTÉO CRITIQUE (CLIMATE SHOCK)\n");
    md.push_str(&format!("- **Niveau d'Alerte :** **`{}`**\n", report.weather_alert.alert_level));
    md.push_str(&format!("- **Type d'Aléa :** `{}`\n", report.weather_alert.hazard_type));
    md.push_str(&format!("- **Condition Déclenchante :** {}\n", report.weather_alert.trigger_condition));
    md.push_str(&format!("- **Protocole d'Urgence :** {}\n\n", report.weather_alert.emergency_protocol));

    md.push_str("## 3. TÉLÉMÉTRIE CARTOGRAPHIQUE & BANQUE NASA\n");
    md.push_str(&format!("- **Corps Planétaire :** `{:?}` | **Grille MGRS :** `{}`\n", report.cartography.body, report.cartography.grid_code));
    md.push_str(&format!("- **Coordonnées :** `{:.4}, {:.4}` (Altitude: `{} m`)\n", report.cartography.lat, report.cartography.lon, report.cartography.elevation_m));
    md.push_str(&format!("- **Source Dataset NASA :** `{}`\n", report.bank_record.source_dataset));
    md.push_str(&format!("- **Humidité Sols/Régolithe :** `{}%` | **Température Surface :** `{}°C`\n\n", report.bank_record.soil_moisture_pct, report.bank_record.surface_temp_c));

    md.push_str("## 4. COPILOTE PRÉDICTIF (HORIZONS 7, 14, 30 JOURS)\n");
    for f in &report.predictive_forecasts {
        md.push_str(&format!("- **J+{} :** Humidité projetée `{}%` | Risque Sécheresse `{}%`  \n  *Action :* {}\n", f.horizon_days, f.projected_soil_moisture_pct, f.drought_risk_index, f.recommended_action_window));
    }
    md.push_str("\n");

    md.push_str("## 5. ALIGNEMENT AVEC LES 10 PRIX INTERNATIONAUX DE LA NASA\n");
    for p in &report.pillars {
        md.push_str(&format!("### [{}] {}\n", p.pillar_id, p.pillar_name));
        md.push_str(&format!("- **Prix Ciblés :** {}\n", p.target_awards.join(" • ")));
        md.push_str(&format!("- **Diagnostic :** {}\n", p.diagnostic));
        md.push_str(&format!("- **Action Prioritaire :** {}\n\n", p.recommended_action));
    }

    let md_path = format!("reports/{}.md", report.report_id);
    let json_path = format!("reports/{}.json", report.report_id);
    fs::write(&md_path, &md)?;
    fs::write(&json_path, serde_json::to_string_pretty(report).unwrap_or_default())?;
    Ok(md_path)
}
