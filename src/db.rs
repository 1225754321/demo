use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;
use tokio::fs as tokiofs;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RouterNode {
    key: String,
    pub name: String,          // 注册的name,相当于id
    pub method: String,        // 请求方式
    pub path: String,          // 请求路由
    pub code: i32,             // 默认返回的code
    pub script: String, // 返回值脚本,会将req传入, 返回str和code都会直接返回,返回obj则是返回json
    pub header_script: String, // 请求头设置值脚本,会将当返回头和req传入,当该脚本返回值不为headers,则使用原来的headers
    pub code_script: String,   // 返回code脚本,当该脚本返回值不为数字时会使用code
    pub is_html: bool, // 是否为html,因为要使用到template,为html的时候script会嵌入到html页面上
    pub template: String, // html模板, 模板name
}

impl RouterNode {
    pub fn key(&self) -> String {
        format!("{}:{}", self.method, self.path)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Template {
    pub name: String,          // 注册的name,相当于id
    pub templage_html: String, // html模板数据
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Datas {
    pub templates: HashMap<String, Template>,
    pub api_routers: HashMap<String, RouterNode>,
}

async fn read_yaml_file(file_path: &str) -> (Datas, SystemTime) {
    let mut file = File::open(file_path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    let metadata = fs::metadata(file_path).unwrap();
    let now_time = metadata.modified().unwrap();
    if contents.is_empty() {
        contents = "templates:\napi_routers:".to_string()
    }
    (serde_yaml::from_str(&contents).unwrap(), now_time)
}
async fn write_yaml_file(file_path: &str, data: &Datas) -> SystemTime {
    let contents = serde_yaml::to_string(data).unwrap();
    let mut file = File::create(file_path).await.unwrap();
    file.write_all(contents.as_bytes()).await.unwrap();
    fs::metadata(file_path).unwrap().modified().unwrap()
}
fn check_file_modified(file_path: &str, old_time: SystemTime) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        if let Ok(modified_time) = metadata.modified() {
            return modified_time > old_time;
        }
    }
    false
}

#[derive(Debug, Clone)]
pub struct DB {
    pub datas: Datas,
    pub old_time: SystemTime,
    pub filepath: String,
}

pub static DB_PATH: &str = "./db.yaml";

impl DB {
    pub async fn new_default() -> DB {
        DB::new(DB_PATH.to_string()).await
    }

    pub async fn new(filepath: String) -> DB {
        if tokiofs::metadata(filepath.clone()).await.is_err() {
            let mut file = tokiofs::File::create(filepath.clone()).await.unwrap();
            file.write_all(b"").await.unwrap();
        }
        let (datas, old_time) = read_yaml_file(&filepath).await;
        DB {
            datas,
            old_time,
            filepath,
        }
    }
    pub async fn read_all(&mut self) -> &Datas {
        if check_file_modified(&self.filepath, self.old_time) {
            let (datas, old_time) = read_yaml_file(&self.filepath).await;
            self.datas = datas;
            self.old_time = old_time;
        }
        &self.datas
    }
    pub async fn update(&mut self) {
        let old_time = write_yaml_file(&self.filepath, &self.datas).await;
        self.old_time = old_time;
    }
    pub async fn set_api(&mut self, mut router_node: RouterNode) {
        router_node.key = router_node.key();
        self.datas
            .api_routers
            .insert(router_node.key.clone(), router_node);
        self.update().await;
    }
    pub async fn set_template(&mut self, template: Template) {
        self.datas.templates.insert(template.name.clone(), template);
        self.update().await;
    }
    pub async fn remove_api(&mut self, key: &str) {
        if let Some(_) = self.datas.api_routers.remove(key) {
            self.update().await;
        }
    }
    pub async fn remove_template(&mut self, name: &str) {
        if let Some(_) = self.datas.templates.remove(name) {
            self.update().await;
        }
    }
}
