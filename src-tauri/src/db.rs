use std::sync::Arc;

use log::info;
use rbatis::{
    crud,
    executor::Executor,
    impl_select_page, impled, py_sql,
    rbdc::datetime::DateTime,
    table_sync::{self, ColumMapper},
    Error, RBatis,
};
use rbdc_sqlite::Driver;
use rbs::Value;
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
    pub name: Option<String>,
    pub path: Option<String>,
    pub describe: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LabelRelationship {
    pub id: Option<String>,
    pub parent: Option<String>,
    pub child: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordLabels {
    pub id: Option<String>,
    pub record: Option<String>,
    pub label: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordQuote {
    pub id: Option<String>,
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
impl_select_page!(Record { select_page(id:Option<String>,content:Option<String>,record_ids:Option<Vec<String>>,create_time_limit:Option<DateLimit>,update_time_limit:Option<DateLimit>) => "
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
"});
crud!(Label {});
impl Label {
    #[py_sql(
        "`select * as count from label where 1=1 `
            if name != null && name != '':
                ` and name like '%'||#{name}||'%' `"
    )]
    pub async fn like(rb: &dyn Executor, name: Option<String>) -> Result<Vec<Label>, Error> {
        impled!()
    }
    pub async fn likes(rb: &dyn Executor, label: Option<String>) -> Option<Vec<String>> {
        let lals = Label::like(rb, label.clone())
            .await
            .unwrap()
            .into_iter()
            .map(|v| v.na);
        if label.is_none() && label.clone().unwrap().len() == 0 {
            let rls = RecordLabels::like(rb, label.clone()).await.unwrap();
            if rls.len() > 10 {
                return Some(rls[0..10].into_iter().map(|v| v.0.clone()).collect());
            }
        }
        None
    }
}
crud!(LabelRelationship {});
crud!(RecordLabels {});
impl RecordLabels {
    #[py_sql(
        "`select label, count(label) as count from record_labels where 1=1 `
            if label != null && label != '':
                ` and label like '%'||#{label}||'%' `
         ` group by label order by count desc`"
    )]
    pub async fn like(
        rb: &dyn Executor,
        label: Option<String>,
    ) -> Result<Vec<(String, usize)>, Error> {
        impled!()
    }
}
crud!(RecordQuote {});
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
        LabelRelationship,
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
        )
        .await
        .unwrap();
        assert_eq!(result7.records.len(), 2);
        assert_eq!(result7.records, vec![record1, record2]);
    }
}
