// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;

use tracing::Level;

mod clap;
mod commands;
mod db;
mod graph;

#[tokio::main]
async fn main() {
    clap::init();

    {
        let cli = clap::CLI.get().unwrap().lock().await;
        tracing_subscriber::fmt()
            .event_format(
                tracing_subscriber::fmt::format()
                    .with_ansi(true)
                    .with_file(true)
                    .with_line_number(true)
                    .compact(),
            )
            .with_max_level(Level::from_str(cli.log.clone().unwrap().as_str()).unwrap())
            .with_level(true)
            .init();
    }

    db::init().await;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_default_config,
            commands::post_records,
            commands::post_record,
            commands::post_labels,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
