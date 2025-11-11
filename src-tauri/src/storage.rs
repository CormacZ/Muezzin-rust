use tauri::AppHandle;
use tauri_plugin_store::{Store, StoreExt};
use std::sync::Arc;
use parking_lot::Mutex;

use crate::models::{AppSettings, CustomTimes, JumuahTime};
use crate::error::Result;

pub struct AppStorage {
    store: Arc<Mutex<Store>>,
}

impl AppStorage {
    pub fn new(app: &AppHandle) -> Result<Self> {
        let store = app.store("settings.json")?;

        Ok(Self {
            store: Arc::new(Mutex::new(store)),
        })
    }

    pub fn get_settings(&self) -> Result<AppSettings> {
        let store = self.store.lock();
        let settings: AppSettings = store
            .get("settings")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        Ok(settings)
    }

    pub fn save_settings(&self, settings: &AppSettings) -> Result<()> {
        let mut store = self.store.lock();
        store.set("settings", serde_json::to_value(settings)?)?;
        store.save()?;
        Ok(())
    }

    pub fn get_location(&self) -> Result<(f64, f64, String)> {
        let store = self.store.lock();
        let lat = store.get("latitude")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let lon = store.get("longitude")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let tz = store.get("timezone")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "UTC".to_string());
        Ok((lat, lon, tz))
    }

    pub fn save_location(&self, lat: f64, lon: f64, timezone: &str) -> Result<()> {
        let mut store = self.store.lock();
        store.set("latitude", serde_json::json!(lat))?;
        store.set("longitude", serde_json::json!(lon))?;
        store.set("timezone", serde_json::json!(timezone))?;
        store.save()?;
        Ok(())
    }

    pub fn get_custom_times(&self) -> Result<Option<CustomTimes>> {
        let store = self.store.lock();
        let custom_times = store
            .get("customTimes")
            .and_then(|v| serde_json::from_value(v.clone()).ok());
        Ok(custom_times)
    }

    pub fn save_custom_times(&self, custom_times: &CustomTimes) -> Result<()> {
        let mut store = self.store.lock();
        store.set("customTimes", serde_json::to_value(custom_times)?)?;
        store.save()?;
        Ok(())
    }

    pub fn get_jumuah_time(&self) -> Result<Option<JumuahTime>> {
        let store = self.store.lock();
        let jumuah_time = store
            .get("jumuahTime")
            .and_then(|v| serde_json::from_value(v.clone()).ok());
        Ok(jumuah_time)
    }

    pub fn save_jumuah_time(&self, jumuah_time: &JumuahTime) -> Result<()> {
        let mut store = self.store.lock();
        store.set("jumuahTime", serde_json::to_value(jumuah_time)?)?;
        store.save()?;
        Ok(())
    }

    pub fn is_first_time(&self) -> bool {
        let store = self.store.lock();
        !store.has("first")
    }

    pub fn set_first_time_done(&self) -> Result<()> {
        let mut store = self.store.lock();
        store.set("first", serde_json::json!(true))?;
        store.save()?;
        Ok(())
    }
}
