#[cfg_attr(mobile, tauri::mobile_entry_point)]

use tauri::{
    utils::config::WebviewUrl,
    webview::{NewWindowResponse, WebviewWindowBuilder},
};

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Set up logging for debug builds
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("Fast Calander Manger")
                .center()
                .resizable(true)
                .maximized(true)
                .on_new_window(|_,_| {
                    // Customize new window creation
                        NewWindowResponse::Allow // Allow new windows for HTTP URLs
                })
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}