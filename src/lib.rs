use serde::Deserialize;
use std::sync::Mutex;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::MrysInit;
#[cfg(mobile)]
use mobile::MrysInit;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the mrys-init APIs.
pub trait MrysInitExt<R: Runtime> {
    fn mrys_init(&self) -> &MrysInit<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MrysInitExt<R> for T {
    fn mrys_init(&self) -> &MrysInit<R> {
        self.state::<MrysInit<R>>().inner()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub webview_label: String,
    pub init_path: String,
}

#[derive(Debug, Clone)]
pub struct AppInitState(bool, Config);

/// Initializes the plugin.
pub fn init<R: Runtime>(init_state: bool) -> TauriPlugin<R, Config> {
    Builder::<R, Config>::new("mrys-init")
        .invoke_handler(tauri::generate_handler![commands::ping])
        .setup(move |app, api| {
            #[cfg(mobile)]
            let mrys_init = mobile::init(app, api)?;
            #[cfg(desktop)]
            let mrys_init = desktop::init(app, api, init_state)?;
            app.manage(mrys_init);
            Ok(())
        })
        .on_page_load(|webview, window| {
            let app_init_state = webview.state::<Mutex<AppInitState>>();
            let init_state = app_init_state.lock().unwrap().clone();
            if webview.label() != init_state.1.webview_label {
                return;
            }
            drop(app_init_state);
            let url = webview.url().unwrap();
            if url.path() == init_state.1.init_path {
                return;
            }
            if !init_state.0 {
                // 未初始化 跳转
                webview
                    .eval(&format!(
                        r#"
                    window.location.pathname = "{}";
                    "#,
                        init_state.1.init_path
                    ))
                    .expect("页面跳转失败");
            }
        })
        .build()
}
