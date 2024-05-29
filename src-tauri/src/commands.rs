use std::collections::{HashMap, HashSet};

use crate::{
    clap,
    db::{DateLimit, Label, Record, RecordLabels, RecordQuote, DB},
};
use log::info;
use rbatis::{rbdc::DateTime, Page, PageRequest};
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
        let data = bodys.data.clone().unwrap();
        get_id = data.id;
        get_content = data.content;
    }
    let select_page;
    let mut qe_map = HashMap::new();
    let mut re_map = HashMap::new();
    let mut rl_map = HashMap::new();
    {
        let lock = DB.get().unwrap().lock().await;
        let mut record_set = HashSet::new();
        let mut irs = false;
        if bodys.data.is_some() {
            let data = bodys.data.unwrap();
            if data.quotes.is_some() && data.quotes.clone().unwrap().len() != 0 {
                for rq in RecordQuote::select_in_column(
                    &lock.to_owned(),
                    "referenced",
                    &data.quotes.unwrap(),
                )
                .await
                .unwrap()
                {
                    record_set.insert(rq.quote.unwrap());
                }
                irs = true;
            }
            if data.referenceds.is_some() && data.referenceds.clone().unwrap().len() != 0 {
                for rq in RecordQuote::select_in_column(
                    &lock.to_owned(),
                    "quote",
                    &data.referenceds.unwrap(),
                )
                .await
                .unwrap()
                {
                    record_set.insert(rq.referenced.unwrap());
                }
                irs = true;
            }
            if data.labels.is_some() && data.labels.clone().unwrap().len() != 0 {
                for rl in
                    RecordLabels::select_in_column(&lock.to_owned(), "label", &data.labels.unwrap())
                        .await
                        .unwrap()
                {
                    record_set.insert(rl.record.unwrap());
                }
                irs = true;
            }
        }
        let record_list: Vec<String> = record_set.into_iter().collect();
        let mut record_ids = None;
        if record_list.len() != 0 {
            record_ids = Some(record_list);
        }
        info!("record_ids => {} {:?}", irs, &record_ids);
        if irs && record_ids.is_none() {
            select_page = Some(Page::new(0, 0))
        } else {
            select_page = Some(
                Record::select_page(
                    &lock.to_owned(),
                    &PageRequest::new(bodys.page.unwrap(), bodys.per_page.unwrap()),
                    get_id,
                    get_content,
                    record_ids,
                    bodys.create_time,
                    bodys.update_time,
                )
                .await
                .unwrap(),
            );
        }
        info!("select_page {:?}", select_page);
        let ids: Vec<String> = select_page
            .clone()
            .unwrap()
            .records
            .into_iter()
            .map(|v| v.id.unwrap())
            .collect();
        for qe in RecordQuote::select_in_column(&lock.to_owned(), "quote", &ids)
            .await
            .unwrap()
        {
            let rq = qe.clone();
            let quote = rq.quote.unwrap();
            if qe_map.contains_key(&quote) {
                qe_map.insert(quote.clone(), Vec::new());
            }
            qe_map
                .get_mut(&quote)
                .unwrap()
                .push(rq.referenced.unwrap().clone());
        }
        for re in RecordQuote::select_in_column(&lock.to_owned(), "referenced", &ids)
            .await
            .unwrap()
        {
            let rq = re.clone();
            let referenced = rq.referenced.unwrap();
            if re_map.contains_key(&referenced) {
                re_map.insert(referenced.clone(), Vec::new());
            }
            re_map
                .get_mut(&referenced)
                .unwrap()
                .push(rq.quote.unwrap().clone());
        }
        for rl in RecordLabels::select_in_column(&lock.to_owned(), "record", &ids)
            .await
            .unwrap()
        {
            let rl = rl.clone();
            let record = rl.record.unwrap();
            if rl_map.contains_key(&record) {
                rl_map.insert(record.clone(), Vec::new());
            }
            rl_map
                .get_mut(&record)
                .unwrap()
                .push(rl.label.unwrap().clone());
        }
    }
    Some(PageRes {
        items: select_page
            .clone()
            .unwrap()
            .records
            .iter()
            .map(|v| {
                let id = v.id.clone().unwrap();
                let mut quotes = None;
                if qe_map.get(&id).is_some() {
                    quotes = qe_map.get(&id).map(|v| v.clone());
                }
                let mut referenceds = None;
                if re_map.get(&id).is_some() {
                    referenceds = re_map.get(&id).map(|v| v.clone());
                }
                let mut labels = None;
                if rl_map.get(&id).is_some() {
                    labels = rl_map.get(&id).map(|v| v.clone());
                }
                RecordVO {
                    id: v.id.clone(),
                    content: v.content.clone(),
                    quotes,
                    referenceds,
                    labels,
                    create_time: v.create_time.clone(),
                    update_time: v.update_time.clone(),
                }
            })
            .collect(),
        total: select_page.unwrap().total,
    })
}

#[tauri::command]
pub async fn post_label(req: Req<Value, String>) -> Option<Vec<String>> {
    info!("post_label {:?}", req);
    {
        let lock = clap::CLI.get().unwrap().lock().await;
    }
    None
}
