import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import * as PrayerAPI from './api.js';
import * as UI from './ui.js';

// Global state
let prayerTimes = null;
let settings = null;
let nextPrayer = null;
let countdownInterval = null;

// Initialize app
async function init() {
    console.log('Initializing Muezzin...');
    
    try {
        // Initialize first time setup if needed
        const isFirstTime = await PrayerAPI.initializeFirstTime();
        console.log('First time:', isFirstTime);

        // Load settings
        settings = await PrayerAPI.getSettings();
        console.log('Settings loaded:', settings);

        // Load prayer times
        await loadPrayerTimes();

        // Setup UI
        UI.renderPrayerTimes(prayerTimes, nextPrayer);
        
        // Start countdown
        startCountdown();

        // Setup event listeners
        setupEventListeners();

        console.log('Muezzin initialized successfully');
    } catch (error) {
        console.error('Initialization error:', error);
        UI.showError('Failed to initialize: ' + error);
    }
}

async function loadPrayerTimes() {
    try {
        prayerTimes = await PrayerAPI.getPrayerTimes();
        nextPrayer = await PrayerAPI.getNextPrayer();
        console.log('Prayer times loaded:', prayerTimes);
        console.log('Next prayer:', nextPrayer);
    } catch (error) {
        console.error('Error loading prayer times:', error);
        throw error;
    }
}

function startCountdown() {
    if (countdownInterval) {
        clearInterval(countdownInterval);
    }

    countdownInterval = setInterval(async () => {
        try {
            const [prayerName, prayerTimeStr] = await PrayerAPI.getNextPrayer();
            const prayerTime = new Date(prayerTimeStr);
            const now = new Date();
            const diff = prayerTime - now;

            if (diff <= 0) {
                // Prayer time passed, reload
                await loadPrayerTimes();
                UI.renderPrayerTimes(prayerTimes, nextPrayer);
            } else {
                UI.updateCountdown(diff);
            }
        } catch (error) {
            console.error('Countdown error:', error);
        }
    }, 1000);
}

function setupEventListeners() {
    // Listen for prayer time updates
    listen('prayers-updated', async () => {
        console.log('Prayer times updated');
        await loadPrayerTimes();
        UI.renderPrayerTimes(prayerTimes, nextPrayer);
    });

    // Listen for navigation events
    listen('navigate-to-settings', () => {
        console.log('Navigate to settings');
        UI.showSettingsPage();
    });
}

// Wait for DOM to be ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}

// Export for debugging
window.muezzin = {
    reload: async () => {
        await loadPrayerTimes();
        UI.renderPrayerTimes(prayerTimes, nextPrayer);
    },
    settings: () => settings,
    prayerTimes: () => prayerTimes,
    nextPrayer: () => nextPrayer
};
