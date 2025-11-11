import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export async function getPrayerTimes() {
    return await invoke('get_prayer_times');
}

export async function getPrayerTimesForDate(date) {
    return await invoke('get_prayer_times_for_date', { date });
}

export async function updateLocation(lat, lon, timezone) {
    return await invoke('update_location', { lat, lon, timezone });
}

export async function getSettings() {
    return await invoke('get_settings');
}

export async function updateSettings(settings) {
    return await invoke('update_settings', { settings });
}

export async function playAdhan() {
    return await invoke('play_adhan');
}

export async function stopAdhan() {
    return await invoke('stop_adhan');
}

export async function setVolume(volume) {
    return await invoke('set_volume', { volume });
}

export async function isAudioPlaying() {
    return await invoke('is_audio_playing');
}

export async function getQiblaDirection() {
    return await invoke('get_qibla_direction');
}

export async function getNextPrayer() {
    return await invoke('get_next_prayer');
}

export async function checkForUpdates() {
    return await invoke('check_for_updates');
}

export async function initializeFirstTime() {
    return await invoke('initialize_first_time');
}

export async function updateCustomTimes(customTimes) {
    return await invoke('update_custom_times', { customTimes });
}

export async function updateJumuahTime(jumuahTime) {
    return await invoke('update_jumuah_time', { jumuahTime });
}

// Event listeners
export function onPrayerTime(callback) {
    return listen('prayer-time', callback);
}

export function onReminderTime(callback) {
    return listen('reminder-time', callback);
}

export function onPrayersUpdated(callback) {
    return listen('prayers-updated', callback);
}
