#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

pub mod builds;
pub mod cmd;
pub mod commands;
pub mod rune_window;
pub mod state;
pub mod web;
pub mod ws;

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub message: String,
}

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle_window", "Toggle"))
        .add_item(CustomMenuItem::new("apply_builds", "Apply Builds"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit").accelerator("CmdOrControl+Q"));

    let context = tauri::generate_context!();
    let mut lcu_client = ws::LcuClient::new();

    let _app = tauri::Builder::default()
        .manage(state::GlobalState::init())
        .setup(move |app| {
            let handle = app.handle();
            let _id = app.listen_global("toggle_rune-global", move |event| {
                println!("global listener, payload {:?}", event.payload().unwrap());
                rune_window::toggle(&handle);
            });

            println!(
                "[state] init app state: {:?}",
                app.state::<state::GlobalState>().0.lock().unwrap()
            );

            // cmd::start_lcu_task(app.state());
            async_std::task::spawn(async move {
                lcu_client.start_lcu_task().await;
            });

            Ok(())
        })
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(move |app_handle, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "toggle_window" => {
                    let _ = rune_window::toggle(app_handle);
                }
                "apply_builds" => {
                    println!("[tray] apply builds");
                    let w = app_handle.get_window("main").unwrap();
                    builds::spawn_apply_task(
                        vec!["op.gg-aram".to_string()],
                        "../.cdn_files".to_string(),
                        false,
                        &w,
                    )
                }
                _ => {
                    println!("{}", id.as_str());
                }
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            commands::greeting,
            commands::toggle_rune_window,
            commands::apply_builds_from_sources,
            commands::get_lcu_auth,
        ])
        .run(context)
        .expect("error while running tauri application");
}
