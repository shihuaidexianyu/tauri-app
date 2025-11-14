use std::sync::{Arc, Mutex};

use crate::{config::AppConfig, models::ApplicationInfo};

#[derive(Default)]
pub struct AppState {
    pub app_index: Arc<Mutex<Vec<ApplicationInfo>>>,
    pub config: Arc<Mutex<AppConfig>>,
    pub registered_hotkey: Arc<Mutex<Option<String>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            app_index: Arc::new(Mutex::new(Vec::new())),
            config: Arc::new(Mutex::new(AppConfig::default())),
            registered_hotkey: Arc::new(Mutex::new(None)),
        }
    }
}
