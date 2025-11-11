use tauri::State;
use chrono::{DateTime, Local};
use crate::{AppState, models::*, error::Result, geolocation};

#[tauri::command]
pub async fn get_prayer_times(state: State<'_, AppState>) -> Result<PrayerTimes> {
    let calculator = state.calculator.read();
    let now = Local::now();
    calculator.calculate_prayer_times(now)
}

#[tauri::command]
pub async fn get_prayer_times_for_date(
    date: String,
    state: State<'_, AppState>,
) -> Result<PrayerTimes> {
    let calculator = state.calculator.read();
    let date = DateTime::parse_from_rfc3339(&date)
        .map_err(|e| crate::error::AppError::Custom(format!("Invalid date format: {}", e)))?
        .with_timezone(&Local);
    calculator.calculate_prayer_times(date)
}

#[tauri::command]
pub async fn update_location(
    lat: f64,
    lon: f64,
    timezone: String,
    state: State<'_, AppState>,
) -> Result<()> {
    state.storage.save_location(lat, lon, &timezone)?;
    
    let tz: chrono_tz::Tz = timezone.parse()
        .map_err(|e| crate::error::AppError::Timezone(format!("Invalid timezone: {}", e)))?;
    
    let settings = state.storage.get_settings()?;
    let custom_times = state.storage.get_custom_times()?;
    let jumuah_time = state.storage.get_jumuah_time()?;
    
    let mut calculator = state.calculator.write();
    calculator.update_settings(lat, lon, &settings.calculation, tz, custom_times, jumuah_time)?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings> {
    state.storage.get_settings()
}

#[tauri::command]
pub async fn update_settings(
    settings: AppSettings,
    state: State<'_, AppState>,
) -> Result<()> {
    state.storage.save_settings(&settings)?;
    
    // Update calculator if needed
    let (lat, lon, tz_str) = state.storage.get_location()?;
    let tz: chrono_tz::Tz = tz_str.parse()
        .map_err(|e| crate::error::AppError::Timezone(format!("Invalid timezone: {}", e)))?;
    
    let custom_times = state.storage.get_custom_times()?;
    let jumuah_time = state.storage.get_jumuah_time()?;
    
    let mut calculator = state.calculator.write();
    calculator.update_settings(lat, lon, &settings.calculation, tz, custom_times, jumuah_time)?;
    
    Ok(())
}

#[tauri::command]
pub async fn play_adhan(state: State<'_, AppState>) -> Result<()> {
    let settings = state.storage.get_settings()?;
    let mut player = state.audio_player.write();
    player.play_adhan(&settings.adhan_path)?;
    Ok(())
}

#[tauri::command]
pub async fn stop_adhan(state: State<'_, AppState>) -> Result<()> {
    let mut player = state.audio_player.write();
    player.stop();
    Ok(())
}

#[tauri::command]
pub async fn set_volume(volume: f32, state: State<'_, AppState>) -> Result<()> {
    let mut player = state.audio_player.write();
    player.set_volume(volume);
    Ok(())
}

#[tauri::command]
pub async fn is_audio_playing(state: State<'_, AppState>) -> Result<bool> {
    let player = state.audio_player.read();
    Ok(player.is_playing())
}

#[tauri::command]
pub async fn get_qibla_direction(state: State<'_, AppState>) -> Result<f64> {
    let calculator = state.calculator.read();
    calculator.get_qibla_direction()
}

#[tauri::command]
pub async fn get_next_prayer(state: State<'_, AppState>) -> Result<(String, String)> {
    let calculator = state.calculator.read();
    let (name, time) = calculator.get_next_prayer()?;
    Ok((name, time.to_rfc3339()))
}

#[tauri::command]
pub async fn check_for_updates() -> Result<Option<String>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/repos/CormacZ/Muezzin-rust/releases/latest")
        .header("User-Agent", "Muezzin")
        .send()
        .await?;
    
    if response.status().is_success() {
        let release: serde_json::Value = response.json().await?;
        if let Some(tag) = release["tag_name"].as_str() {
            return Ok(Some(tag.to_string()));
        }
    }
    
    Ok(None)
}

#[tauri::command]
pub async fn initialize_first_time(state: State<'_, AppState>) -> Result<bool> {
    if state.storage.is_first_time() {
        println!("First time launch detected, fetching location...");
        
        // Get location from IP
        match geolocation::get_location_from_ip().await {
            Ok(location_info) => {
                println!("Location fetched: {:?}", location_info);
                
                // Save location
                state.storage.save_location(
                    location_info.latitude,
                    location_info.longitude,
                    &location_info.timezone
                )?;

                // Set default calculation method based on location
                let mut settings = state.storage.get_settings()?;
                if let (Some(continent), Some(country)) = (&location_info.continent_code, &location_info.country_code) {
                    settings.calculation.calc_method = geolocation::get_default_calculation_method(continent, country);
                }
                state.storage.save_settings(&settings)?;

                // Initialize calculator
                let tz: chrono_tz::Tz = location_info.timezone.parse()
                    .map_err(|e| crate::error::AppError::Timezone(format!("Invalid timezone: {}", e)))?;
                
                let custom_times = state.storage.get_custom_times()?;
                let jumuah_time = state.storage.get_jumuah_time()?;
                
                let mut calculator = state.calculator.write();
                calculator.update_settings(
                    location_info.latitude,
                    location_info.longitude,
                    &settings.calculation,
                    tz,
                    custom_times,
                    jumuah_time
                )?;

                state.storage.set_first_time_done()?;
                Ok(true)
            },
            Err(e) => {
                eprintln!("Error fetching location: {}", e);
                Err(e)
            }
        }
    } else {
        // Not first time, load existing settings
        let (lat, lon, tz_str) = state.storage.get_location()?;
        let settings = state.storage.get_settings()?;
        let custom_times = state.storage.get_custom_times()?;
        let jumuah_time = state.storage.get_jumuah_time()?;
        
        let tz: chrono_tz::Tz = tz_str.parse()
            .map_err(|e| crate::error::AppError::Timezone(format!("Invalid timezone: {}", e)))?;
        
        let mut calculator = state.calculator.write();
        calculator.update_settings(lat, lon, &settings.calculation, tz, custom_times, jumuah_time)?;
        
        Ok(false)
    }
}

#[tauri::command]
pub async fn update_custom_times(
    custom_times: CustomTimes,
    state: State<'_, AppState>,
) -> Result<()> {
    state.storage.save_custom_times(&custom_times)?;
    
    // Reload calculator settings
    let (lat, lon, tz_str) = state.storage.get_location()?;
    let settings = state.storage.get_settings()?;
    let jumuah_time = state.storage.get_jumuah_time()?;
    
    let tz: chrono_tz::Tz = tz_str.parse()
        .map_err(|e| crate::error::AppError::Timezone(format!("Invalid timezone: {}", e)))?;
    
    let mut calculator = state.calculator.write();
    calculator.update_settings(lat, lon, &settings.calculation, tz, Some(custom_times), jumuah_time)?;
    
    Ok(())
}

#[tauri::command]
pub async fn update_jumuah_time(
    jumuah_time: JumuahTime,
    state: State<'_, AppState>,
) -> Result<()> {
    state.storage.save_jumuah_time(&jumuah_time)?;
    
    // Reload calculator settings
    let (lat, lon, tz_str) = state.storage.get_location()?;
    let settings = state.storage.get_settings()?;
    let custom_times = state.storage.get_custom_times()?;
    
    let tz: chrono_tz::Tz = tz_str.parse()
        .map_err(|e| crate::error::AppError::Timezone(format!("Invalid timezone: {}", e)))?;
    
    let mut calculator = state.calculator.write();
    calculator.update_settings(lat, lon, &settings.calculation, tz, custom_times, Some(jumuah_time))?;
    
    Ok(())
}
