use tauri::{AppHandle, Manager, tray::{TrayIconBuilder, TrayIconEvent}, menu::{MenuBuilder, MenuItemBuilder}};
use crate::error::Result;

pub fn setup_tray(app: &AppHandle) -> Result<()> {
    let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
    let open_item = MenuItemBuilder::with_id("open", "Open").build(app)?;
    let settings_item = MenuItemBuilder::with_id("settings", "Settings").build(app)?;
    
    let menu = MenuBuilder::new(app)
        .item(&open_item)
        .separator()
        .item(&settings_item)
        .separator()
        .item(&quit_item)
        .build()?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(move |app, event| {
            match event.id().as_ref() {
                "quit" => {
                    app.exit(0);
                }
                "open" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "settings" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        // Emit event to navigate to settings
                        let _ = window.emit("navigate-to-settings", ());
                    }
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|_tray, event| {
            if let TrayIconEvent::Click { button, .. } = event {
                if button == tauri::tray::MouseButton::Left {
                    // Left click - show window
                    println!("Tray icon left clicked");
                }
            }
        })
        .build(app)?;

    Ok(())
}

pub fn update_tray_menu(app: &AppHandle, prayer_times: &crate::models::PrayerTimes) -> Result<()> {
    // TODO: Update tray menu with current prayer times
    // This can be called when prayer times are updated
    Ok(())
}
