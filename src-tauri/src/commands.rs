use crate::clap;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Req<P: Serialize + Clone, B: Serialize + Clone> {
    params: Option<P>,
    bodys: Option<B>,
}

#[tauri::command]
pub async fn get_default_config() -> clap::Cli {
    let lock = clap::CLI.get().unwrap().lock().await;
    lock.clone()
}

#[tauri::command]
pub async fn get_records() -> clap::Cli {
    let lock = clap::CLI.get().unwrap().lock().await;
    lock.clone()
}
