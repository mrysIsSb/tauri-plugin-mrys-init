use crate::models::*;
use crate::{AppInitState, Config};
use std::sync::Mutex;
use tauri::{plugin::PluginApi, AppHandle, Manager, Runtime};

pub fn init<R: Runtime>(
    app: &AppHandle<R>,
    api: PluginApi<R, Config>,
    init_state: bool,
) -> crate::Result<MrysInit<R>> {
    let config = api.config();
    println!("mrys-init init: {:?}", config);
    app.manage(Mutex::new(AppInitState(init_state, config.clone())));
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

    //是否初始化
    pub fn is_init(&self) -> crate::Result<bool> {
        let app_init_state = self.0.state::<Mutex<AppInitState>>();
        let app_init_state = app_init_state.lock().unwrap();
        Ok(app_init_state.0)
    }
}
