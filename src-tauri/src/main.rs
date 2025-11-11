#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod prayer_times;
mod storage;
mod audio;
mod geolocation;
mod models;
mod commands;
mod error;
mod tray;

use tauri::{Manager, State};
use tauri_plugin_autostart::MacosLauncher;
use std::sync::Arc;
use parking_lot::RwLock;

use crate::storage::AppStorage;
use crate::prayer_times::PrayerCalculator;
use crate::audio::AudioPlayer;

pub struct AppState {
    storage: Arc<AppStorage>,
    calculator: Arc<RwLock<PrayerCalculator>>,
    audio_player: Arc<RwLock<AudioPlayer>>,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .setup(|app| {
            let storage = Arc::new(AppStorage::new(app.handle()).expect("Failed to initialize storage"));
            let calculator = Arc::new(RwLock::new(PrayerCalculator::new()));
            let audio_player = Arc::new(RwLock::new(AudioPlayer::new().expect("Failed to initialize audio")));

            app.manage(AppState {
                storage: storage.clone(),
                calculator: calculator.clone(),
                audio_player: audio_player.clone(),
            });

            // Setup system tray
            tray::setup_tray(app.handle()).expect("Failed to setup tray");

            // Initialize prayer time checker
            let handle = app.handle().clone();
            tokio::spawn(async move {
                prayer_times::start_prayer_checker(storage, calculator, audio_player, handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_prayer_times,
            commands::get_prayer_times_for_date,
            commands::update_location,
            commands::update_settings,
            commands::get_settings,
            commands::play_adhan,
            commands::stop_adhan,
            commands::get_qibla_direction,
            commands::check_for_updates,
            commands::initialize_first_time,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
