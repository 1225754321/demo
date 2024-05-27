use clap::Parser;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};
/// myrecord
#[derive(Parser, Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[command(version="0.0.2", author, about, long_about = None)]
pub struct Cli {
    /// log级别
    #[arg(short, long, default_value = "info")]
    pub log: Option<String>,

    /// 文件存储位置
    #[arg(short = 'f', long, default_value = "./target/files/")]
    pub files: Option<String>,

    /// home页默认显示页面 enum{list,chart}
    #[arg(long, default_value = "list")]
    pub default_home: Option<String>,

    /// chart默认显示数据类型 enum{quote,label}
    #[arg(long, default_value = "quote")]
    pub default_chart: Option<String>,
}

pub static CLI: OnceCell<Arc<Mutex<Cli>>> = OnceCell::const_new();

pub fn init() {
    CLI.set(Arc::new(Mutex::new(Cli::parse()))).unwrap();
}
