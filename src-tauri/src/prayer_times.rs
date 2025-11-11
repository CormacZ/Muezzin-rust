use salah::prelude::*;
use chrono::{DateTime, Local, Timelike, Datelike, NaiveTime};
use chrono_tz::Tz;
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::time::{sleep, Duration};
use tauri::{AppHandle, Manager};

use crate::storage::AppStorage;
use crate::models::{PrayerTimes, CalculationSettings, CustomTimes, JumuahTime};
use crate::audio::AudioPlayer;
use crate::error::{AppError, Result};

pub struct PrayerCalculator {
    coordinates: Option<Coordinates>,
    config: Option<Configuration>,
    timezone: Tz,
    custom_times: Option<CustomTimes>,
    jumuah_time: Option<JumuahTime>,
}

impl PrayerCalculator {
    pub fn new() -> Self {
        Self {
            coordinates: None,
            config: None,
            timezone: chrono_tz::UTC,
            custom_times: None,
            jumuah_time: None,
        }
    }

    pub fn update_settings(
        &mut self,
        lat: f64,
        lon: f64,
        calc_settings: &CalculationSettings,
        timezone: Tz,
        custom_times: Option<CustomTimes>,
        jumuah_time: Option<JumuahTime>,
    ) -> Result<()> {
        self.coordinates = Some(Coordinates::new(lat, lon));
        self.timezone = timezone;
        self.custom_times = custom_times;
        self.jumuah_time = jumuah_time;
        
        // Map calculation method
        let method = match calc_settings.calc_method.as_str() {
            "MWL" => Method::MuslimWorldLeague,
            "Egyptian" => Method::Egyptian,
            "Karachi" => Method::Karachi,
            "UAQ" => Method::UmmAlQura,
            "Dubai" => Method::Dubai,
            "Qatar" => Method::Qatar,
            "Kuwait" => Method::Kuwait,
            "MC" => Method::MoonsightingCommittee,
            "Singapore" => Method::Singapore,
            "Turkey" => Method::Turkey,
            "Tehran" => Method::Tehran,
            "ISNA" => Method::NorthAmerica,
            _ => Method::MuslimWorldLeague,
        };

        let mut config = Configuration::with(method);
        
        // Set madhab
        config.madhab = match calc_settings.madhab.as_str() {
            "Hanafi" => Madhab::Hanafi,
            _ => Madhab::Shafi,
        };

        // Apply adjustments if any
        if let Some(ref adj) = calc_settings.adjustments {
            config.fajr_offset = adj.fajr;
            config.dhuhr_offset = adj.dhuhr;
            config.asr_offset = adj.asr;
            config.maghrib_offset = adj.maghrib;
            config.isha_offset = adj.isha;
        }

        self.config = Some(config);
        Ok(())
    }

    pub fn calculate_prayer_times(&self, date: DateTime<Local>) -> Result<PrayerTimes> {
        let coords = self.coordinates.ok_or(AppError::NotInitialized)?;
        let config = self.config.as_ref().ok_or(AppError::NotInitialized)?;

        let prayers = salah::PrayerSchedule::new()
            .on(date.date_naive())
            .for_location(coords)
            .with_configuration(config.clone())
            .calculate()
            .map_err(|e| AppError::Prayer(e.to_string()))?;

        let mut result = PrayerTimes {
            fajr: prayers.time(Prayer::Fajr).with_timezone(&self.timezone),
            sunrise: prayers.time(Prayer::Sunrise).with_timezone(&self.timezone),
            dhuhr: prayers.time(Prayer::Dhuhr).with_timezone(&self.timezone),
            asr: prayers.time(Prayer::Asr).with_timezone(&self.timezone),
            maghrib: prayers.time(Prayer::Maghrib).with_timezone(&self.timezone),
            isha: prayers.time(Prayer::Isha).with_timezone(&self.timezone),
        };

        // Apply custom times if enabled and it's today
        if let Some(ref custom) = self.custom_times {
            if custom.enabled && date.date_naive() == Local::now().date_naive() {
                if let Some(ref time_str) = custom.fajr {
                    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
                        result.fajr = date.date_naive().and_time(time).and_local_timezone(self.timezone).unwrap();
                    }
                }
                if let Some(ref time_str) = custom.dhuhr {
                    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
                        result.dhuhr = date.date_naive().and_time(time).and_local_timezone(self.timezone).unwrap();
                    }
                }
                if let Some(ref time_str) = custom.asr {
                    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
                        result.asr = date.date_naive().and_time(time).and_local_timezone(self.timezone).unwrap();
                    }
                }
                if let Some(ref time_str) = custom.maghrib {
                    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
                        result.maghrib = date.date_naive().and_time(time).and_local_timezone(self.timezone).unwrap();
                    }
                }
                if let Some(ref time_str) = custom.isha {
                    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
                        result.isha = date.date_naive().and_time(time).and_local_timezone(self.timezone).unwrap();
                    }
                }
            }
        }

        // Apply Jumuah time if it's Friday
        if let Some(ref jumuah) = self.jumuah_time {
            if jumuah.enabled && date.weekday() == chrono::Weekday::Fri {
                if let Ok(time) = NaiveTime::parse_from_str(&jumuah.time, "%H:%M") {
                    result.dhuhr = date.date_naive().and_time(time).and_local_timezone(self.timezone).unwrap();
                }
            }
        }

        Ok(result)
    }

    pub fn get_next_prayer(&self) -> Result<(String, DateTime<Local>)> {
        let now = Local::now();
        let times = self.calculate_prayer_times(now)?;

        if now < times.fajr {
            Ok(("Fajr".to_string(), times.fajr))
        } else if now < times.dhuhr {
            Ok(("Dhuhr".to_string(), times.dhuhr))
        } else if now < times.asr {
            Ok(("Asr".to_string(), times.asr))
        } else if now < times.maghrib {
            Ok(("Maghrib".to_string(), times.maghrib))
        } else if now < times.isha {
            Ok(("Isha".to_string(), times.isha))
        } else {
            // Next is Fajr tomorrow
            let tomorrow = now + chrono::Duration::days(1);
            let tomorrow_times = self.calculate_prayer_times(tomorrow)?;
            Ok(("Fajr".to_string(), tomorrow_times.fajr))
        }
    }

    pub fn get_qibla_direction(&self) -> Result<f64> {
        let coords = self.coordinates.ok_or(AppError::NotInitialized)?;
        Ok(coords.qibla_direction())
    }
}

pub async fn start_prayer_checker(
    storage: Arc<AppStorage>,
    calculator: Arc<RwLock<PrayerCalculator>>,
    audio_player: Arc<RwLock<AudioPlayer>>,
    app_handle: AppHandle,
) {
    let mut check_interval = tokio::time::interval(Duration::from_secs(1));
    let mut last_check_minute = 999u32;
    let mut last_date = Local::now().date_naive();

    loop {
        check_interval.tick().await;
        
        let now = Local::now();
        let current_minute = now.hour() * 60 + now.minute();
        let current_date = now.date_naive();

        // Recalculate prayers at midnight
        if current_date != last_date {
            last_date = current_date;
            // Emit event to frontend to refresh prayers
            let _ = app_handle.emit("prayers-updated", ());
        }

        // Only check once per minute
        if current_minute == last_check_minute {
            continue;
        }
        last_check_minute = current_minute;

        // Load settings
        let settings = match storage.get_settings() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error loading settings: {}", e);
                continue;
            }
        };

        if !settings.adhan_check && !settings.notif_check {
            continue;
        }

        // Get next prayer
        let calc = calculator.read();
        let (prayer_name, prayer_time) = match calc.get_next_prayer() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error getting next prayer: {}", e);
                continue;
            }
        };
        drop(calc);

        // Check if it's prayer time (within 1 second)
        let diff = (prayer_time.timestamp() - now.timestamp()).abs();
        if diff <= 1 {
            println!("Prayer time! {}", prayer_name);
            
            // Play adhan
            if settings.adhan_check {
                let mut player = audio_player.write();
                let adhan_path = if prayer_name == "Fajr" && settings.adhan_fajr_path.is_some() {
                    settings.adhan_fajr_path.as_ref().unwrap()
                } else {
                    &settings.adhan_path
                };
                
                if let Err(e) = player.play_adhan(adhan_path) {
                    eprintln!("Error playing adhan: {}", e);
                }
            }

            // Show notification
            if settings.notif_check {
                let _ = app_handle.notification()
                    .builder()
                    .title("Prayer Time")
                    .body(format!("It's time for {} prayer", prayer_name))
                    .show();
            }
        }

        // Check for reminders
        if let Some(ref reminder) = settings.reminder_times {
            if reminder.enabled {
                let minutes_until = ((prayer_time.timestamp() - now.timestamp()) / 60) as u32;
                
                let reminder_minutes = match prayer_name.as_str() {
                    "Fajr" => reminder.fajr,
                    "Dhuhr" => {
                        if now.weekday() == chrono::Weekday::Fri {
                            reminder.jumuah
                        } else {
                            reminder.dhuhr
                        }
                    },
                    "Asr" => reminder.asr,
                    "Maghrib" => reminder.maghrib,
                    "Isha" => reminder.isha,
                    _ => 0,
                };

                if reminder_minutes > 0 && minutes_until == reminder_minutes {
                    if settings.notif_check {
                        let _ = app_handle.notification()
                            .builder()
                            .title("Prayer Reminder")
                            .body(format!("Adhan in {} minutes", reminder_minutes))
                            .show();
                    }
                }
            }
        }
    }
}
