use crate::{
    clap,
    db::{DateLimit, Record, DB},
};
use log::info;
use rbatis::{rbdc::DateTime, PageRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Req<P: Serialize + Clone, B: Serialize + Clone> {
    params: Option<P>,
    bodys: Option<B>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordVO {
    id: Option<String>,
    content: Option<String>,
    quotes: Option<Vec<String>>,
    referenceds: Option<Vec<String>>,
    labels: Option<Vec<String>>,
    create_time: Option<DateTime>,
    update_time: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordPage {
    page: Option<String>,
    #[serde(rename = "perPage")]
    per_page: Option<String>,
    id: Option<String>,
    content: Option<String>,
    quotes: Option<Vec<String>>,
    referenceds: Option<Vec<String>>,
    labels: Option<Vec<String>>,
    create_time: Option<DateLimit>,
    update_time: Option<DateLimit>,
}

#[tauri::command]
pub async fn get_default_config() -> clap::Cli {
    let lock = clap::CLI.get().unwrap().lock().await;
    lock.clone()
}

#[tauri::command]
pub async fn get_record(req: Req<RecordPage, Value>) -> Option<Vec<RecordVO>> {
    info!("req => {:?}", req);
    {
        let lock = DB.get().unwrap().lock().await;
        let select_page = Record::select_page(
            &lock.to_owned(),
            &PageRequest::new(0, 2),
            Some(&Record {
                id: Some("%t%".to_string()),
                content: Some("".to_string()),
                create_time: None,
                update_time: None,
            }),
            None,
            None,
        )
        .await
        .unwrap();
        info!("{:?}", select_page);
    }
    None
}
