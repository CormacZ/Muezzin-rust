// UI rendering functions

export function renderPrayerTimes(prayerTimes, nextPrayer) {
    const app = document.getElementById('app');
    
    if (!prayerTimes) {
        app.innerHTML = `
            <div class="loading-screen">
                <div class="spinner-border text-primary" role="status">
                    <span class="visually-hidden">Loading...</span>
                </div>
            </div>
        `;
        return;
    }

    const [nextPrayerName, nextPrayerTime] = nextPrayer || ['', ''];
    
    const prayers = [
        { name: 'Fajr', time: prayerTimes.fajr, icon: 'fa-moon' },
        { name: 'Sunrise', time: prayerTimes.sunrise, icon: 'fa-sun' },
        { name: 'Dhuhr', time: prayerTimes.dhuhr, icon: 'fa-sun' },
        { name: 'Asr', time: prayerTimes.asr, icon: 'fa-cloud-sun' },
        { name: 'Maghrib', time: prayerTimes.maghrib, icon: 'fa-sunset' },
        { name: 'Isha', time: prayerTimes.isha, icon: 'fa-moon' }
    ];

    const prayerCardsHTML = prayers.map(prayer => {
        const isNext = prayer.name === nextPrayerName;
        const time = new Date(prayer.time).toLocaleTimeString('en-US', {
            hour: '2-digit',
            minute: '2-digit',
            hour12: true
        });

        return `
            <div class="col-md-6 col-lg-4">
                <div class="prayer-card ${isNext ? 'next-prayer' : ''}">
                    <div class="d-flex align-items-center justify-content-between">
                        <div>
                            <div class="prayer-name">
                                <i class="fas ${prayer.icon} me-2"></i>
                                ${prayer.name}
                            </div>
                            <div class="prayer-time">${time}</div>
                            ${isNext ? '<div class="countdown" id="countdown">Next prayer</div>' : ''}
                        </div>
                    </div>
                </div>
            </div>
        `;
    }).join('');

    app.innerHTML = `
        <div class="container-fluid h-100 p-4">
            <div class="row mb-4">
                <div class="col">
                    <h1 class="text-center mb-3">
                        <i class="fas fa-mosque me-2"></i>
                        Muezzin
                    </h1>
                    <p class="text-center text-muted">
                        ${new Date().toLocaleDateString('en-US', {
                            weekday: 'long',
                            year: 'numeric',
                            month: 'long',
                            day: 'numeric'
                        })}
                    </p>
                </div>
            </div>
            
            <div class="row g-3">
                ${prayerCardsHTML}
            </div>

            <div class="row mt-4">
                <div class="col text-center">
                    <button class="btn btn-primary me-2" onclick="window.muezzin.reload()">
                        <i class="fas fa-sync me-2"></i>Refresh
                    </button>
                    <button class="btn btn-secondary" onclick="window.muezzin.settings()">
                        <i class="fas fa-cog me-2"></i>Settings
                    </button>
                </div>
            </div>
        </div>
    `;
}

export function updateCountdown(milliseconds) {
    const countdownEl = document.getElementById('countdown');
    if (!countdownEl) return;

    const hours = Math.floor(milliseconds / (1000 * 60 * 60));
    const minutes = Math.floor((milliseconds % (1000 * 60 * 60)) / (1000 * 60));
    const seconds = Math.floor((milliseconds % (1000 * 60)) / 1000);

    countdownEl.textContent = `In ${hours}h ${minutes}m ${seconds}s`;
}

export function showError(message) {
    const app = document.getElementById('app');
    app.innerHTML = `
        <div class="container h-100 d-flex align-items-center justify-content-center">
            <div class="text-center">
                <i class="fas fa-exclamation-triangle fa-4x text-danger mb-3"></i>
                <h2>Error</h2>
                <p class="text-muted">${message}</p>
                <button class="btn btn-primary" onclick="location.reload()">
                    <i class="fas fa-redo me-2"></i>Reload
                </button>
            </div>
        </div>
    `;
}

export function showSettingsPage() {
    const app = document.getElementById('app');
    app.innerHTML = `
        <div class="container-fluid h-100 p-4">
            <div class="row mb-4">
                <div class="col">
                    <h1>
                        <button class="btn btn-link text-white" onclick="location.reload()">
                            <i class="fas fa-arrow-left"></i>
                        </button>
                        Settings
                    </h1>
                </div>
            </div>
            
            <div class="row">
                <div class="col">
                    <div class="alert alert-info">
                        <i class="fas fa-info-circle me-2"></i>
                        Settings page coming soon!
                    </div>
                </div>
            </div>
        </div>
    `;
}
