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

impl RecordVO {
    fn get_id(&self) -> Option<String> {
        if self.id.is_some() {
            Some(format!("%{}%", self.id.clone().unwrap()))
        } else {
            None
        }
    }
    fn get_content(&self) -> Option<String> {
        if self.id.is_some() {
            Some(format!("%{}%", self.content.clone().unwrap()))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PageReq<T> {
    page: Option<u64>,
    per_page: Option<u64>,
    create_time: Option<DateLimit>,
    update_time: Option<DateLimit>,
    data: Option<T>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PageRes<T> {
    items: Vec<T>,
    total: u64,
}

#[tauri::command]
pub async fn get_default_config() -> clap::Cli {
    let lock = clap::CLI.get().unwrap().lock().await;
    lock.clone()
}

#[tauri::command]
pub async fn post_record(req: Req<Value, PageReq<RecordVO>>) -> Option<PageRes<RecordVO>> {
    info!("req => {:?}", req);
    let bodys = req.bodys.unwrap();
    let (mut get_id, mut get_content) = (None, None);
    if bodys.data.is_some() {
        get_id = bodys.data.as_ref().unwrap().get_id();
        get_content = bodys.data.unwrap().get_content();
    }
    let mut select_page = None;
    {
        let lock = DB.get().unwrap().lock().await;
        select_page = Some(
            Record::select_page(
                &lock.to_owned(),
                &PageRequest::new(bodys.page.unwrap(), bodys.per_page.unwrap()),
                get_id,
                get_content,
                bodys.create_time,
                bodys.update_time,
            )
            .await
            .unwrap(),
        );
        info!("select_page {:?}", select_page);
    }
    Some(PageRes {
        items: select_page
            .clone()
            .unwrap()
            .records
            .iter()
            .map(|v| {
                let quotes = Vec::new();
                let referenceds = Vec::new();
                let labels = Vec::new();
                RecordVO {
                    id: v.id.clone(),
                    content: v.content.clone(),
                    quotes: Some(quotes),
                    referenceds: Some(referenceds),
                    labels: Some(labels),
                    create_time: v.create_time.clone(),
                    update_time: v.update_time.clone(),
                }
            })
            .collect(),
        total: select_page.unwrap().total,
    })
}
