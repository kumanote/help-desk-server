use job_config::AppConfig;
use once_cell::sync::OnceCell;
use std::sync::Arc;

static GLOBAL_APP_CONFIG: OnceCell<Arc<AppConfig>> = OnceCell::new();

pub fn app_config() -> &'static AppConfig {
    GLOBAL_APP_CONFIG
        .get()
        .expect("app config must have been set...")
}

pub fn set_app_config(app_config: AppConfig) {
    if GLOBAL_APP_CONFIG.set(Arc::new(app_config)).is_err() {
        eprintln!("Global app config has already been set");
    }
}
