use std::{collections::HashSet, sync::Arc};

use log::info;
use rbatis::{
    crud,
    executor::Executor,
    impl_select_page, impled, py_sql,
    rbdc::{datetime::DateTime, db::ExecResult},
    table_sync::{self, ColumMapper},
    Error, RBatis,
};
use rbdc_sqlite::Driver;
use rbs::to_value;
use tokio::sync::{Mutex, OnceCell};

/// table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default, PartialEq, Eq)]
pub struct Record {
    pub id: Option<String>,
    pub content: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Label {
    pub id: Option<String>,
    pub describe: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordLabels {
    pub record: Option<String>,
    pub label: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordQuote {
    pub quote: Option<String>,
    pub referenced: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordFile {
    pub id: Option<String>,
    pub quote: Option<String>,
    pub referenced: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct File {
    pub id: Option<String>,
    pub name: Option<String>,
    pub md5: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct DateLimit {
    pub start_time: Option<DateTime>,
    pub end_time: Option<DateTime>,
}

crud!(Record {});
impl_select_page!(Record { select_page(
    id:Option<String>,content:Option<String>,record_ids:Option<Vec<String>>,order_by:Option<String>,order_dir:Option<String>,create_time_limit:Option<DateLimit>,update_time_limit:Option<DateLimit>) => "
`where 1 = 1`
if id != null && id != '':
    ` and id like '%'||#{id}||'%'`
if content != null && content != '':
    ` and content like '%'||#{content}||'%'`
if record_ids != null && record_ids.len != 0:
    ` and id in #{record_ids}`
if create_time_limit != null:
    if create_time_limit.start_time != null:
        ` and create_time >= #{create_time_limit.start_time}`
    if create_time_limit.end_time != null:
        ` and create_time <= #{create_time_limit.end_time}`
if update_time_limit != null:
    if update_time_limit.start_time != null:
        ` and update_time >= #{update_time_limit.start_time}`
    if update_time_limit.end_time != null:
        ` and update_time <= #{update_time_limit.end_time}`
if order_by != null && order_by != '' && order_dir != null && order_dir != '':
    ` order by ${order_by} ${order_dir}`
"});
crud!(Label {});
impl Label {
    #[py_sql(
        "`select * as count from label where 1=1 `
            if id != null && id != '':
                ` and id like '%'||#{id}||'%' `"
    )]
    pub async fn like(rb: &dyn Executor, id: Option<String>) -> Result<Vec<Label>, Error> {
        impled!()
    }
    // #[py_sql(
    //     "`insert into label values `
    //     for i,v in ids:
    //         ` (${v},'',date('now'),date('now')) `
    //         if i != ids.len - 1:
    //             `,`
    //     "
    // )]
    pub async fn adds(rb: &dyn Executor, ids: Option<Vec<String>>) -> Result<ExecResult, Error> {
        let mut sql = "insert into label values ".to_string();
        let mut args = Vec::new();
        let len = ids.clone().unwrap().len() - 1;
        for (i, ele) in ids.unwrap().into_iter().enumerate() {
            sql.push_str(" (?, '', date('now'), date('now') ) ");
            args.push(to_value!(ele.clone()));
            if i != len {
                sql.push(',');
            }
        }
        rb.exec(&sql, args).await
    }

    pub async fn likes(rb: &dyn Executor, id: Option<String>) -> Option<Vec<String>> {
        let ids: Vec<String> = Label::like(rb, id.clone())
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|v| v.id.unwrap())
            .collect();
        let rls = RecordLabels::label_like(rb, id.clone())
            .await
            .unwrap_or_default();
        let mut rl_ids = rls
            .into_iter()
            .map(|v| v.0.clone())
            .collect::<HashSet<String>>();
        if id.is_none() || id.clone().unwrap().trim().is_empty() {
            for id in ids.iter() {
                if rl_ids.len() >= 10 {
                    break;
                }
                rl_ids.insert(id.to_string());
            }
        } else {
            for id in ids.iter() {
                rl_ids.insert(id.to_string());
            }
        }

        if id.is_none() && rl_ids.len() >= 10 {
            Some(rl_ids.into_iter().collect::<Vec<String>>()[0..10].to_vec())
        } else {
            Some(rl_ids.into_iter().collect())
        }
    }
}
crud!(RecordLabels {});
impl RecordLabels {
    #[py_sql(
        "`select label, count(label) as count from record_labels where 1=1 `
            if label != null && label != '':
                ` and label like '%'||#{label}||'%' `
         ` group by label order by count desc`"
    )]
    pub async fn label_like(
        rb: &dyn Executor,
        label: Option<String>,
    ) -> Result<Vec<(String, usize)>, Error> {
        impled!()
    }

    pub async fn add_record_labels(
        rb: &dyn Executor,
        record: Option<String>,
        labels: Option<Vec<String>>,
    ) -> Result<ExecResult, Error> {
        let mut sql = "insert into record_labels values ".to_string();
        let mut args = Vec::new();
        let record = record.unwrap();
        let len = labels.clone().unwrap().len() - 1;
        for (i, ele) in labels.unwrap().into_iter().enumerate() {
            sql.push_str(" (?, ?, date('now'), date('now') ) ");
            args.push(to_value!(record.clone()));
            args.push(to_value!(ele.clone()));
            if i != len {
                sql.push(',');
            }
        }
        rb.exec(&sql, args).await
    }
}
crud!(RecordQuote {});
impl RecordQuote {
    pub async fn add_record_quotes(
        rb: &dyn Executor,
        record: Option<String>,
        quotes: Option<Vec<String>>,
    ) -> Result<ExecResult, Error> {
        let mut sql = "insert into record_quote values ".to_string();
        let mut args = Vec::new();
        let record = record.unwrap();
        let len = quotes.clone().unwrap().len() - 1;
        for (i, ele) in quotes.unwrap().into_iter().enumerate() {
            sql.push_str(" (?, ?, date('now'), date('now') ) ");
            args.push(to_value!(record.clone()));
            args.push(to_value!(ele.clone()));
            if i != len {
                sql.push(',');
            }
        }
        rb.exec(&sql, args).await
    }
}
crud!(RecordFile {});
crud!(File {});

fn pascal_to_camel(pascal_str: &str) -> String {
    let mut camel_str = String::new();
    for (i, ch) in pascal_str.chars().enumerate() {
        if i != 0 && ch.is_uppercase() {
            camel_str.push('_');
        }
        camel_str.push(ch.to_lowercase().next().unwrap());
    }
    camel_str
}

macro_rules! sync_tables {
    ($r:tt,$m:tt, $( $x:ident ),* ) => {
        $(
            RBatis::sync(
                &$r.acquire().await.unwrap(),
                $m.clone(),
                &$x::default(),
                &pascal_to_camel(stringify!($x)),
            )
            .await
            .unwrap();
        )*
    };
}

pub static DB: OnceCell<Arc<Mutex<RBatis>>> = OnceCell::const_new();

pub async fn init() {
    if DB.get().is_some() {
        return;
    }

    let rb = RBatis::new();

    rb.link(Driver {}, "sqlite:./target/db.sqlite")
        .await
        .unwrap();

    let mapper = &table_sync::SqliteTableMapper {} as &dyn ColumMapper;
    sync_tables!(
        rb,
        mapper,
        Record,
        Label,
        RecordLabels,
        RecordQuote,
        RecordFile,
        File
    );

    DB.get_or_init(|| async { Arc::new(Mutex::new(rb)) }).await;

    info!("db start end");
}

mod tests {

    #[tokio::test]
    async fn db_test3() {
        use super::*;
        use rbatis::PageRequest;

        init().await;
        let db = DB.get().unwrap().lock().await;

        db.exec("delete from Record", vec![]).await.unwrap();

        // Create test data
        let record1 = Record {
            id: Some("1".to_string()),
            content: Some("content 1".to_string()),
            create_time: Some(
                DateTime::parse("YYYY-MM-DD hh:mm:ss.000000", "2024-05-01 19:19:12.8703204")
                    .unwrap(),
            ),
            update_time: Some(
                DateTime::parse("YYYY-MM-DD hh:mm:ss.000000", "2024-05-02 19:19:12.8703204")
                    .unwrap(),
            ),
        };
        Record::insert(&db.to_owned(), &record1).await.unwrap();

        let record2 = Record {
            id: Some("2".to_string()),
            content: Some("content 2".to_string()),
            create_time: Some(
                DateTime::parse("YYYY-MM-DD hh:mm:ss.000000", "2024-05-10 19:19:12.8703204")
                    .unwrap(),
            ),
            update_time: Some(
                DateTime::parse("YYYY-MM-DD hh:mm:ss.000000", "2024-05-11 19:19:12.8703204")
                    .unwrap(),
            ),
        };
        Record::insert(&db.to_owned(), &record2).await.unwrap();

        // Call the select_page method with different arguments to cover different code paths
        let page_request = PageRequest::new(0, 10);
        let date_limit = DateLimit {
            start_time: Some(
                DateTime::parse("YYYY-MM-DD hh:mm:ss.000000", "2024-05-09 19:19:12.8703204")
                    .unwrap(),
            ),
            end_time: Some(
                DateTime::parse("YYYY-MM-DD hh:mm:ss.000000", "2024-05-23 19:19:12.8703204")
                    .unwrap(),
            ),
        };

        // Test with id filter
        let result1 = Record::select_page(
            &db.to_owned(),
            &page_request,
            Some("1".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
        assert_eq!(result1.records.len(), 1);
        assert_eq!(result1.records[0], record1);

        // Test with content filter
        let result2 = Record::select_page(
            &db.to_owned(),
            &page_request,
            None,
            Some("content 2".to_string()),
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
        assert_eq!(result2.records.len(), 1);
        assert_eq!(result2.records[0], record2);

        // Test with create_time_limit filter
        let result3 = Record::select_page(
            &db.to_owned(),
            &page_request,
            None,
            None,
            None,
            None,
            None,
            Some(date_limit.clone()),
            None,
        )
        .await
        .unwrap();
        assert_eq!(result3.records.len(), 1);
        assert_eq!(result3.records[0], record2);

        // Test with update_time_limit filter
        let result4 = Record::select_page(
            &db.to_owned(),
            &page_request,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(date_limit.clone()),
        )
        .await
        .unwrap();
        assert_eq!(result4.records.len(), 1);
        assert_eq!(result4.records[0], record2);

        // Test with update_time_limit filter2
        let mut date_limit2 = date_limit.clone();
        date_limit2.end_time = None;

        let result5 = Record::select_page(
            &db.to_owned(),
            &page_request,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(date_limit2),
        )
        .await
        .unwrap();
        assert_eq!(result5.records.len(), 1);
        assert_eq!(result5.records[0], record2);

        let result6 = Record::select_page(
            &db.to_owned(),
            &page_request,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(DateLimit {
                start_time: Some(
                    DateTime::parse("YYYY-MM-DD hh:mm:ss.000000", "2024-05-01 01:19:12.8703204")
                        .unwrap(),
                ),
                end_time: None,
            }),
        )
        .await
        .unwrap();
        assert_eq!(result6.records.len(), 2);
        assert_eq!(result6.records, vec![record1.clone(), record2.clone()]);

        let result7 = Record::select_page(
            &db.to_owned(),
            &page_request,
            None,
            None,
            Some(vec!["1".to_string(), "2".to_string()]),
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
        assert_eq!(result7.records.len(), 2);
        assert_eq!(result7.records, vec![record1, record2]);
    }
}
