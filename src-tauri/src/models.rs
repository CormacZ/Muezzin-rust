use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerTimes {
    pub fajr: DateTime<Local>,
    pub sunrise: DateTime<Local>,
    pub dhuhr: DateTime<Local>,
    pub asr: DateTime<Local>,
    pub maghrib: DateTime<Local>,
    pub isha: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub startup_sound: bool,
    pub notif_check: bool,
    pub systray: bool,
    pub adhan_check: bool,
    pub auto_start: bool,
    pub min_start: bool,
    pub adhan_path: String,
    pub adhan_fajr_path: Option<String>,
    pub dua_enabled: bool,
    pub reminder_times: Option<ReminderTimes>,
    pub calculation: CalculationSettings,
    pub language: String,
    pub dark_mode: bool,
    pub bg_image: Option<BgImage>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            startup_sound: false,
            notif_check: true,
            systray: true,
            adhan_check: true,
            auto_start: true,
            min_start: false,
            adhan_path: "ressources/audio/Adhan - Ahmed Al-Nufais.mp3".to_string(),
            adhan_fajr_path: None,
            dua_enabled: true,
            reminder_times: None,
            calculation: CalculationSettings::default(),
            language: "en".to_string(),
            dark_mode: true,
            bg_image: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BgImage {
    pub enabled: bool,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderTimes {
    pub enabled: bool,
    pub fajr: u32,
    pub dhuhr: u32,
    pub asr: u32,
    pub maghrib: u32,
    pub isha: u32,
    pub jumuah: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CalculationSettings {
    pub calc_method: String,
    pub madhab: String,
    pub hlr: String,
    pub pcr: String,
    pub shafaq: String,
    pub adjustments: Option<Adjustments>,
}

impl Default for CalculationSettings {
    fn default() -> Self {
        Self {
            calc_method: "MWL".to_string(),
            madhab: "Shafi".to_string(),
            hlr: "TA".to_string(),
            pcr: "CC".to_string(),
            shafaq: "shafaqG".to_string(),
            adjustments: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adjustments {
    pub fajr: i32,
    pub dhuhr: i32,
    pub asr: i32,
    pub maghrib: i32,
    pub isha: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTimes {
    pub enabled: bool,
    pub fajr: Option<String>,
    pub dhuhr: Option<String>,
    pub asr: Option<String>,
    pub maghrib: Option<String>,
    pub isha: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumuahTime {
    pub enabled: bool,
    pub time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationInfo {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub continent_code: Option<String>,
    pub country_code: Option<String>,
}
