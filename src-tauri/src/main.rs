use env_logger::Env;
use log::info;
use tauri::Manager;
use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

fn handle_system_tray_event(app_handle: &tauri::AppHandle, event: tauri::SystemTrayEvent) {
    match event {
        tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                if let Some(window) = app_handle.get_window("main") {
                    window.hide().unwrap();
                    let tray_handle = app_handle.tray_handle();
                    tray_handle.get_item("hide").set_enabled(false).unwrap();
                    tray_handle.get_item("show").set_enabled(true).unwrap();
                }
            }
            "show" => {
                if let Some(window) = app_handle.get_window("main") {
                    window.show().unwrap();
                    let tray_handle = app_handle.tray_handle();
                    tray_handle.get_item("show").set_enabled(false).unwrap();
                    tray_handle.get_item("hide").set_enabled(true).unwrap();
                }
            }
            _ => {}
        },
        _ => {}
    }
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting main...");
    let show = CustomMenuItem::new("show".to_string(), "显示").disabled();
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    let app = tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(handle_system_tray_event);

    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}