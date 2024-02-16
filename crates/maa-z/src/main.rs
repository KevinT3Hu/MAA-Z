// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use maa::MaaInstanceAPI;
use tauri::{async_runtime::Mutex, Manager};
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::OffsetTime;

mod callback;
mod commands;
mod config;
mod error;
mod maa;
mod model;
mod task;

#[derive(Clone)]
#[repr(transparent)]
pub struct InstHandle(*mut MaaInstanceAPI);

// Safety: InstHandle is Send and Sync because MaaInstanceAPI is Send and Sync
unsafe impl Send for InstHandle {}
// Safety: InstHandle is Send and Sync because MaaInstanceAPI is Send and Sync
unsafe impl Sync for InstHandle {}

pub type ConfigHolderState = Mutex<config::ConfigHolder>;

fn main() {
    let _guard = init_tracing();

    tracing::info!("Starting Maa");

    #[allow(clippy::expect_used)]
    #[allow(clippy::str_to_string)]
    tauri::Builder::default()
        .setup(|app| {
            let path =
                std::env::current_exe().expect("error while getting current executable path");
            let config_file = path.with_file_name("maa.toml");
            let config =
                config::ConfigHolder::new(config_file).expect("error while reading config file");
            app.manage(Mutex::new(config));

            let handle = maa::get_maa_handle(app.app_handle());
            let inst = InstHandle(handle);

            app.manage(inst);

            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::device::find_devices,
            commands::device::connect_to_device,
            commands::config::change_client_type,
            commands::task::start_up,
            commands::maa::init_maa
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[allow(clippy::absolute_paths)]
fn init_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("logs", "maa.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    #[allow(clippy::expect_used)]
    let timer = OffsetTime::local_rfc_3339().expect("error while creating timer");

    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(Level::TRACE)
        .with_writer(non_blocking)
        .with_timer(timer)
        .init();

    guard
}