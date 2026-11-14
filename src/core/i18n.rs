use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Language {
    French,
    English,
    Darija,
}

impl Language {
    pub fn from_query(q: &str) -> Self {
        let q_lower = q.to_lowercase();
        if q_lower.contains("english") || q_lower.contains("eng") || q_lower.contains("nasa") {
            Language::English
        } else if q_lower.contains("darija") || q_lower.contains("maghribi") || q_lower.contains("darij") || q_lower.contains("lard") || q_lower.contains("ns9iw") || q_lower.contains("chems") || q_lower.contains("ma") {
            Language::Darija
        } else {
            Language::French
        }
    }
}

pub struct Translator;

impl Translator {
    pub fn translate_diagnosis(lang: Language, soil_m: f64, temp: f64, hazard: &str) -> String {
        match lang {
            Language::French => format!(
                "Analyse agronomique : Humidité des sols à {:.1}%, Température de surface à {:.1}°C. Aléa détecté : {}.",
                soil_m, temp, hazard
            ),
            Language::English => format!(
                "Agronomic analysis: Soil moisture at {:.1}%, Surface temperature at {:.1}°C. Hazard detected: {}.",
                soil_m, temp, hazard
            ),
            Language::Darija => {
                let status = if soil_m < 25.0 { "L'ard nشفات و فيها العطش بزاف (Terre sèche et assoiffée)" } else { "الحالة مزيانة و الرطوبة مقادة (État stable)" };
                format!(
                    "تكسير البيانات : رطوبة الأرض {:.1}%، حرارة السطح {:.1}°C. {}. الخطر المكتشف : {}.",
                    soil_m, temp, status, hazard
                )
            },
        }
    }

    pub fn translate_action(lang: Language, action: &str) -> String {
        match lang {
            Language::French => format!("Action recommandée : {}", action),
            Language::English => format!("Recommended action: {}", action),
            Language::Darija => format!(
                "الحل باش نوجدو راسنا : خصنا نديرو سقي بالتنقيط (Goutte-à-goutte) و نغطيوا الأرض بالبايلاج (Paillage) باش ما تنشفش الماء. التفاصيل : {}",
                action
            ),
        }
    }
}
