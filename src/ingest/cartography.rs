use crate::core::models::{GeoCoordinate, PlanetaryBody};
use regex::Regex;

pub struct CartographyEngine;

impl CartographyEngine {
    /// Résout la position automatiquement : détecte GPS, texte, ou applique une géo-tactique de secours souveraine.
    pub fn resolve(query: Option<&str>, lat_opt: Option<f64>, lon_opt: Option<f64>, force_mars: bool) -> GeoCoordinate {
        let (mut lat, mut lon) = (lat_opt, lon_opt);
        let q_lower = query.unwrap_or("").to_lowercase();
        let mut auto_located = false;

        let is_mars = force_mars
            || q_lower.contains("mars")
            || q_lower.contains("jezero")
            || q_lower.contains("gale")
            || q_lower.contains("olympus");

        // Extraction GPS depuis la requête texte si présente
        if (lat.is_none() || lon.is_none()) && query.is_some() {
            let re = Regex::new(r"(-?\d{1,2}\.\d+)[,\s]+(-?\d{1,3}\.\d+)").unwrap();
            if let Some(caps) = re.captures(query.unwrap()) {
                lat = caps.get(1).and_then(|m| m.as_str().parse::<f64>().ok());
                lon = caps.get(2).and_then(|m| m.as_str().parse::<f64>().ok());
                auto_located = true;
            }
        }

        let body = if is_mars { PlanetaryBody::Mars } else { PlanetaryBody::Earth };

        // Si aucune position n'est fournie, géolocalisation automatique sur la position de référence (Casablanca / Cratère Jezero)
        let (final_lat, final_lon) = match (&body, lat, lon) {
            (PlanetaryBody::Mars, None, None) => {
                auto_located = true;
                (18.3800, 77.5800) // Cratère Jezero par défaut
            }
            (PlanetaryBody::Earth, None, None) => {
                auto_located = true;
                (33.5731, -7.5898) // Casablanca / Chaouia par défaut
            }
            (_, Some(la), Some(lo)) => (la, lo),
            (_, Some(la), None) => (la, 0.0),
            (_, None, Some(lo)) => (0.0, lo),
        };

        let elevation_m = match body {
            PlanetaryBody::Earth => (((final_lat.to_radians().sin() * 120.0) + (final_lon.to_radians().cos() * 85.0)).abs() * 10.0).round() / 10.0,
            PlanetaryBody::Mars => -2540.0 + (final_lat.to_radians().sin() * 1400.0).round(),
        };

        let climate_zone = match body {
            PlanetaryBody::Earth => {
                let a = final_lat.abs();
                if a < 15.0 { "Aw/Af - Tropical Humide".into() }
                else if a < 30.0 { "BWh/BSh - Aride à Semi-Aride Chaud".into() }
                else if a < 42.0 { "Csa/BSk - Méditerranéen / Semi-Aride".into() }
                else { "Dfb/Cfb - Tempéré Continental".into() }
            }
            PlanetaryBody::Mars => "MARS-HYPERARID - Cryosphère CO2 (Serre Pressurisée CELSS)".into(),
        };

        let zone = (((final_lon + 180.0) / 6.0).floor() as i32) + 1;
        let band = if final_lat >= 0.0 { 'N' } else { 'S' };
        let prefix = if body == PlanetaryBody::Mars { "MARS" } else { "MGRS" };
        let grid_code = format!(
            "{}-{}{} {:05} {:05}",
            prefix,
            zone,
            band,
            ((final_lon.fract().abs()) * 100_000.0) as u32,
            ((final_lat.fract().abs()) * 100_000.0) as u32
        );

        GeoCoordinate {
            body,
            lat: final_lat,
            lon: final_lon,
            elevation_m,
            climate_zone,
            grid_code,
            auto_located,
        }
    }
}
