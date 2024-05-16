use clap::Parser;

/// urlmock程序
#[derive(Parser)]
#[command(version="0.0.2", author, about, long_about = None)]
pub struct Cli {
    /// host配置
    #[arg(short = 's', long, default_value = "0.0.0.0")]
    pub host: Option<String>,

    /// 端口配置
    #[arg(short, long, default_value = "8080")]
    pub port: Option<u32>,
}
