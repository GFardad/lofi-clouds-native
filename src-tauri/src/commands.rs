use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowState {
    pub width: u32,
    pub height: u32,
    pub is_maximized: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            width: 400,
            height: 600,
            is_maximized: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct WindowStateStore {
    pub path: PathBuf,
}

impl WindowStateStore {
    pub fn new(app: &tauri::App) -> Self {
        let app_data_dir = app.path().app_data_dir().expect("failed to resolve app data dir");
        fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
        let path = app_data_dir.join("window_state.json");
        Self { path }
    }

    pub fn load(&self) -> WindowState {
        if let Ok(data) = fs::read_to_string(&self.path) {
            if let Ok(state) = serde_json::from_str::<WindowState>(&data) {
                return state;
            }
        }
        WindowState::default()
    }

    pub fn save(&self, state: &WindowState) -> Result<(), String> {
        let json = serde_json::to_string(state).map_err(|e| e.to_string())?;
        fs::write(&self.path, json).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn save_window_state(
    state: State<WindowStateStore>,
    width: u32,
    height: u32,
    is_maximized: bool,
) -> Result<(), String> {
    let window_state = WindowState {
        width,
        height,
        is_maximized,
    };
    state.save(&window_state)
}

#[tauri::command]
pub fn load_window_state(state: State<WindowStateStore>) -> WindowState {
    state.load()
}
