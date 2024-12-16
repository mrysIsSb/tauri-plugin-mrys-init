use crate::models::*;
use crate::AppInitState;
use serde::de::DeserializeOwned;
use std::sync::Mutex;
use tauri::{plugin::PluginApi, AppHandle, Listener, Manager, Runtime, WebviewUrl};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<MrysInit<R>> {
    Ok(MrysInit(app.clone()))
}

/// Access to the mrys-init APIs.
pub struct MrysInit<R: Runtime>(AppHandle<R>);

impl<R: Runtime> MrysInit<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }
    // 初始化成功
    pub fn init_success(&self) -> crate::Result<()> {
        let app_init_state = self.0.state::<Mutex<AppInitState>>();
        let mut app_init_state = app_init_state.lock().unwrap();
        app_init_state.0 = true;
        drop(app_init_state);
        Ok(())
    }
}
