use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};

use lofi_clouds_native::commands::{load_window_state, save_window_state, WindowStateStore};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let state = WindowStateStore::new(app);
            app.manage(state);

            let show = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &hide, &quit])?;

            let tray = TrayIconBuilder::new()
                .menu(&menu)
                .menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    let id = event.id().as_ref();
                    match id {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "hide" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.hide();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window("main") {
                let saved = load_window_state(state.clone());
                let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                    width: saved.width,
                    height: saved.height,
                }));
                if saved.is_maximized {
                    let _ = window.maximize();
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                window.hide().ok();
                api.prevent_close();
            }
            WindowEvent::Resized(size) => {
                if let Ok(maximized) = window.is_maximized() {
                    let _ = window.app_handle().invoke::<()>(
                        "save_window_state",
                        tauri::json::json!({
                            "width": size.width,
                            "height": size.height,
                            "is_maximized": maximized
                        }),
                    );
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![save_window_state, load_window_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
