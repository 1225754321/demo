use std::sync::Arc;

use log::info;
//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
use rbatis::{
    crud, impl_select, impl_select_page,
    rbdc::datetime::DateTime,
    table_sync::{self, ColumMapper},
    RBatis,
};
use rbdc_sqlite::Driver;
use tokio::sync::{Mutex, OnceCell};

/// table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
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
impl_select_page!(Record { select_page(record:Option<&Record>,create_time_limit:Option<&DateLimit>,update_time_limit:Option<&DateLimit>) => "
`where 1 = 1`
if record != null:
    if record.id != null && record.id != '':
        ` and id like #{record.id}`
    if record.content != null && record.content != '':
        ` and content like #{record.content}`
if create_time_limit != null && create_time_limit.start_time != null && create_time_limit.end_time != null:
    ` and create_time between #{create_time_limit.start_time} and #{create_time_limit.end_time}`
if update_time_limit != null && update_time_limit.start_time != null && update_time_limit.end_time != null:
    ` and update_time between #{update_time_limit.start_time} and #{update_time_limit.end_time}`
"});

crud!(Label {});
crud!(LabelRelationship {});
crud!(RecordLabels {});
crud!(RecordQuote {});
impl_select!(RecordQuote{select_rq(qs:Option<Vec<String>>,rc:Option<Vec<String>>)=>"
`where 1 = 1`
if qs != null:
    ` and quote in #{rs}`
if rs != null:
    ` and referenced in #{qs}`
"});
crud!(RecordFile {});
crud!(File {});

macro_rules! sync_tables {
    ($r:tt,$m:tt, $( $x:ident ),* ) => {
        $(
            RBatis::sync(
                &$r.acquire().await.unwrap(),
                $m.clone(),
                &$x::default(),
                stringify!($x),
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
    async fn db_test1() {
        use super::*;
        use rbatis::rbdc::Uuid;
        init().await;
        let get = DB.get().unwrap().lock().await;
        Record::insert(
            &get.to_owned(),
            &Record {
                id: Some(Uuid::new().to_string()),
                content: Some("content".to_string()),
                create_time: Some(DateTime::now()),
                update_time: Some(DateTime::now()),
            },
        )
        .await
        .unwrap();
        println!(
            "records 1 => {:?}",
            Record::select_all(&get.to_owned()).await.unwrap()
        );
    }

    #[tokio::test]
    async fn db_test2() {
        use super::*;
        use rbatis::PageRequest;

        init().await;
        let db = DB.get().unwrap().lock().await;
        println!(
            "records 2 => {:?}",
            Record::select_page(
                &db.to_owned(),
                &PageRequest::new(0, 10),
                Some(&Record {
                    id: None,
                    content: Some("%c%".to_string()),
                    create_time: None,
                    update_time: None
                }),
                // Some(&DateLimit {
                //     start_time: Some(
                //         DateTime::parse(
                //             "YYYY-MM-DD hh:mm:ss.000000",
                //             "2024-05-19 19:19:12.8703204"
                //         )
                //         .unwrap()
                //     ),
                //     end_time: Some(
                //         DateTime::parse(
                //             "YYYY-MM-DD hh:mm:ss.000000",
                //             "2024-05-23 19:19:12.8703204"
                //         )
                //         .unwrap()
                //     ),
                // }),
                None,
                None,
            )
            .await
            .unwrap()
        );
    }
}
