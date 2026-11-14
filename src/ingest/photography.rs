use crate::core::models::SpectralTelemetry;
use image::{GenericImageView, Rgb, RgbImage};
use std::path::Path;

pub struct PhotographyEngine;

impl PhotographyEngine {
    pub fn analyze(image_path: Option<&str>) -> SpectralTelemetry {
        if let Some(path_str) = image_path {
            let path = Path::new(path_str);
            if path.exists() {
                if let Ok(img) = image::open(path) {
                    let (w, h) = img.dimensions();
                    let thumb = img.thumbnail(256, 256).to_rgb8();
                    let mut sum_r = 0.0_f64;
                    let mut sum_g = 0.0_f64;
                    let mut sum_b = 0.0_f64;
                    let mut green_pixels = 0_u64;
                    let mut hot_pixels = 0_u64;
                    let total = thumb.pixels().len().max(1) as f64;

                    for p in thumb.pixels() {
                        let (r, g, b) = (p[0] as f64, p[1] as f64, p[2] as f64);
                        sum_r += r;
                        sum_g += g;
                        sum_b += b;

                        let exg = (2.0 * g - r - b) / 255.0;
                        if exg > 0.07 { green_pixels += 1; }
                        if r > g * 1.15 && r > 125.0 { hot_pixels += 1; }
                    }

                    let mean_r = sum_r / total;
                    let mean_g = sum_g / total;
                    let mean_b = sum_b / total;

                    let denom = (mean_g + mean_r - mean_b).max(1.0);
                    let vari = ((mean_g - mean_r) / denom).clamp(-1.0, 1.0);
                    let pseudo_ndvi = ((mean_g - mean_r) / (mean_g + mean_r).max(1.0) * 1.35).clamp(-1.0, 1.0);
                    let canopy_pct = (green_pixels as f64 / total) * 100.0;
                    let thermal_score = (hot_pixels as f64 / total) * 100.0;
                    let water_stress = (1.0 - ((pseudo_ndvi + 1.0) / 2.0)).clamp(0.05, 0.98);

                    return SpectralTelemetry {
                        source_analyzed: true,
                        resolution: (w, h),
                        mean_rgb: [
                            (mean_r * 10.0).round() / 10.0,
                            (mean_g * 10.0).round() / 10.0,
                            (mean_b * 10.0).round() / 10.0,
                        ],
                        vari_index: (vari * 1000.0).round() / 1000.0,
                        pseudo_ndvi: (pseudo_ndvi * 1000.0).round() / 1000.0,
                        water_stress_index: (water_stress * 100.0).round() / 100.0,
                        canopy_coverage_pct: (canopy_pct * 10.0).round() / 10.0,
                        thermal_anomaly_score: (thermal_score * 10.0).round() / 10.0,
                    };
                }
            }
        }

        SpectralTelemetry {
            source_analyzed: false,
            resolution: (0, 0),
            mean_rgb: [138.0, 146.0, 112.0],
            vari_index: 0.046,
            pseudo_ndvi: 0.34,
            water_stress_index: 0.62,
            canopy_coverage_pct: 41.5,
            thermal_anomaly_score: 38.0,
        }
    }

    pub fn generate_sandbox_satellite_capture(output_path: &str) -> std::io::Result<()> {
        let mut img = RgbImage::new(256, 256);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let stress_gradient = ((x as f32 / 256.0) + (y as f32 / 256.0)) * 0.5;
            let r = (65.0 + stress_gradient * 145.0).min(255.0) as u8;
            let g = (165.0 - stress_gradient * 75.0).max(40.0) as u8;
            let b = (55.0 + (x % 16) as f32) as u8;
            *pixel = Rgb([r, g, b]);
        }
        img.save(output_path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}
