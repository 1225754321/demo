use clap::Parser;

/// urlmock程序
#[derive(Parser, Default, Debug)]
#[command(version="0.0.2", author, about, long_about = None)]
pub struct Cli {
    /// host配置
    #[arg(short = 't', long, default_value = "0.0.0.0")]
    pub host: Option<String>,

    /// 端口配置
    #[arg(short, long, default_value = "80")]
    pub port: Option<u32>,

    /// https
    #[arg(short = 's', long, default_value = "true")]
    pub https: Option<bool>,

    /// 证书配置
    #[arg(long, default_value = "0.0.0.0")]
    pub https_host: Option<String>,

    /// 端口配置
    #[arg(long, default_value = "443")]
    pub https_port: Option<u16>,

    /// 证书配置
    #[arg(long, default_value = "./keys/server.crt")]
    pub https_crt: Option<String>,

    /// 密钥配置
    #[arg(long, default_value = "./keys/server.key")]
    pub https_key: Option<String>,
}
