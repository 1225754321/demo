use std::collections::{HashMap, HashSet};

use crate::{
    clap,
    db::{DateLimit, Label, Record, RecordLabels, RecordQuote, DB},
};
use log::{error, info};
use rbatis::{rbdc::DateTime, Page, PageRequest};
use regex::Regex;
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
    order_by: Option<String>,
    order_dir: Option<String>,
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
pub async fn post_records(req: Req<Value, PageReq<RecordVO>>) -> Option<PageRes<RecordVO>> {
    info!("post_records => {:#?}", req);
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
                let temp: Vec<String> = RecordQuote::select_in_column(
                    &lock.to_owned(),
                    "quote",
                    &data.referenceds.unwrap(),
                )
                .await
                .unwrap()
                .into_iter()
                .map(|v| v.referenced.unwrap())
                .collect();
                if irs {
                    record_set = record_set
                        .into_iter()
                        .filter(|v| temp.contains(v))
                        .collect();
                } else {
                    record_set.extend(temp);
                }
                irs = true;
            }
            if data.labels.is_some() && data.labels.clone().unwrap().len() != 0 {
                let temp: Vec<String> = RecordLabels::select_in_column(
                    &lock.to_owned(),
                    "label",
                    &data.labels.unwrap(),
                )
                .await
                .unwrap()
                .into_iter()
                .map(|v| v.record.unwrap())
                .collect();
                if irs {
                    record_set = record_set
                        .into_iter()
                        .filter(|v| temp.contains(v))
                        .collect();
                } else {
                    record_set.extend(temp);
                }
                irs = true;
            }
        }
        info!("record_set => {:#?}", record_set);
        let record_list: Vec<String> = record_set.into_iter().collect();
        let mut record_ids = None;
        if record_list.len() != 0 {
            record_ids = Some(record_list);
        }
        info!("record_ids => {} {:#?}", irs, &record_ids);
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
                    bodys.order_by,
                    bodys.order_dir,
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
            if !qe_map.contains_key(&quote) {
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
            if !re_map.contains_key(&referenced) {
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
            if !rl_map.contains_key(&record) {
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
pub async fn post_labels(req: Req<Value, String>) -> Option<Vec<String>> {
    info!("post_label {:?}", req);
    let labels;
    {
        let lock = DB.get().unwrap().lock().await.to_owned();
        labels = Label::likes(&lock, req.bodys.clone()).await;
    }
    let mut res: HashSet<String> = labels.unwrap_or_default().into_iter().collect();
    if req.bodys.is_some() && !req.bodys.clone().unwrap().trim().is_empty() {
        res.insert(req.bodys.unwrap().trim().to_string());
    }
    Some(res.into_iter().collect())
}

#[tauri::command]
pub async fn post_record(req: Req<Value, RecordVO>) -> Result<(), &'static str> {
    info!("post_record {:?}", req);
    let bodys = req.bodys.unwrap();
    {
        let mut lock = DB
            .get()
            .unwrap()
            .lock()
            .await
            .to_owned()
            .acquire_begin()
            .await
            .unwrap()
            .defer_async(|mut tx| async move {
                if !tx.done {
                    tx.rollback().await.unwrap_or_default();
                    error!("tx rollback success!");
                } else {
                    info!("don't need rollback!");
                }
            });
        if Record::select_by_column(&lock, "id", bodys.id.clone().unwrap())
            .await
            .unwrap()
            .len()
            > 0
        {
            return Err("该id已被使用!");
        };
        let in_labels: Vec<String> =
            Label::select_in_column(&lock, "id", &bodys.labels.clone().unwrap())
                .await
                .unwrap()
                .into_iter()
                .map(|v| v.id.unwrap())
                .collect();
        let not_in_labels: Vec<String> = bodys
            .labels
            .clone()
            .unwrap()
            .into_iter()
            .filter(|v| !in_labels.contains(v))
            .collect();
        if let Err(e) = Label::adds(&lock, Some(not_in_labels.clone())).await {
            error!("adds {}", e);
            return Err("标签添加失败!");
        }
        if let Err(e) =
            RecordLabels::add_record_labels(&lock, Some(bodys.id.clone().unwrap()), bodys.labels)
                .await
        {
            error!("add_record_labels {}", e);
            return Err("标签引用关系添加失败!");
        }
        let reg = Regex::new(r"#R\{(.+?)\}").unwrap();
        let mut quotes = HashSet::new();
        for cap in reg.captures_iter(&bodys.content.clone().unwrap()) {
            quotes.insert(cap.get(1).unwrap().as_str().to_string());
        }
        if let Err(e) = RecordQuote::add_record_quotes(
            &lock,
            Some(bodys.id.clone().unwrap()),
            Some(quotes.into_iter().collect()),
        )
        .await
        {
            error!("add_record_quotes {}", e);
            return Err("记录引用关系添加失败!");
        }
        if let Err(e) = Record::insert(
            &lock,
            &Record {
                id: bodys.id,
                content: bodys.content,
                create_time: Some(DateTime::now()),
                update_time: Some(DateTime::now()),
            },
        )
        .await
        {
            error!("insert {}", e);
            return Err("记录数据添加失败!");
        }
        lock.commit().await.unwrap_or_default();
    }
    Ok(())
}
