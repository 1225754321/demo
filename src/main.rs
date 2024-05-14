use router::Manger;
use salvo::serve_static::StaticDir;
use salvo::{
    http::{mime, HeaderName},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::{Mutex, OnceCell};
pub mod db;
pub mod js;
pub mod router;
pub mod tree;

#[derive(Serialize, Deserialize)]
struct RequestInfo {
    method: String,
    url: String,
    path: String,
    headers: HashMap<String, String>,
    body_utf8: String,
    body_json: HashMap<String, Value>,
    files: Vec<HashMap<String, String>>,
    query_string: String,
    query_map: HashMap<String, Vec<String>>,
}

#[handler]
async fn manger_run(req: &mut Request, resp: &mut Response) -> String {
    let mut files: Vec<HashMap<String, String>> = Vec::new();
    let mut from_utf8: String = String::new();
    let mut from_json = HashMap::new();

    // 获取router_node
    let router_node: db::RouterNode;
    {
        let manger = MANGER.get().unwrap().lock().await;
        if let Some(temp) = manger.query(&req.method().to_string(), &req.uri().path().to_string()) {
            router_node = temp.clone();
        } else {
            resp.status_code(StatusCode::NOT_FOUND);
            return "".to_string();
        }
    }

    if let Some(ctype) = req.content_type() {
        if ctype.subtype() == mime::FORM_DATA {
            if let Ok(temp) = req.form_data().await {
                from_utf8 = mime::FORM_DATA.to_string();
                for (_, v) in temp.files.clone() {
                    files.append(
                        &mut v
                            .into_iter()
                            .map(|f| {
                                let mut temp_map = HashMap::new();
                                temp_map.insert(
                                    "filename".to_string(),
                                    f.name().unwrap_or_default().to_string(),
                                );
                                if let Some(temp_get) = f.headers().get("content-disposition") {
                                    let var = temp_get.to_str().unwrap_or_default().to_string();
                                    temp_map.insert("content-disposition".to_string(), var);
                                }
                                if let Some(temp_get) = f.headers().get("content-type") {
                                    let var = temp_get.to_str().unwrap_or_default().to_string();
                                    temp_map.insert("content-type".to_string(), var);
                                }

                                temp_map
                            })
                            .collect(),
                    )
                }
            }
            if let Ok(parse_from) = req.parse_form().await {
                from_json = parse_from;
            }
        }
    }

    let r = req.payload().await.unwrap();
    if let Ok(temp) = String::from_utf8(r.to_vec()) {
        if !temp.is_empty() {
            from_utf8 = temp;
        }
    }
    if let Ok(parse_json) = req.parse_json::<HashMap<String, Value>>().await {
        if !parse_json.is_empty() {
            from_json = parse_json;
        }
    }

    let req_info = RequestInfo {
        method: req.method().to_string(),
        url: req.uri().to_string(),
        path: req.uri().path().to_string(),
        headers: req
            .headers()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
            .collect(),
        body_utf8: from_utf8,
        body_json: from_json,
        files: files,
        query_string: req.uri().query().unwrap_or("").to_string(),
        query_map: req
            .queries()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect(),
    };
    let mut run_js_data = Option::None;
    if !router_node.script.is_empty() {
        let run_js_script = format!(
            "req={};{}",
            serde_json::to_string(&req_info).unwrap(),
            router_node.script
        );
        println!("run_js_script.script => {}\n", &run_js_script);
        run_js_data = Some(js::run_js(run_js_script));
        println!("run_js_script.script.data => {:?}\n", &run_js_data);
    }

    // 设置code
    if !router_node.code_script.is_empty() {
        let run_js_script = format!(
            "req={};{}",
            serde_json::to_string(&req_info).unwrap(),
            router_node.code_script
        );
        println!("run_js_script.code => {}\n", &run_js_script);
        let script_data = js::run_js(run_js_script);
        println!("run_js_script.code.data => {}\n", script_data);
        let code = script_data.as_i64().unwrap();
        let code = StatusCode::from_u16(code as u16).unwrap();
        resp.status_code(code);
    }

    // 设置headers
    if !router_node.header_script.is_empty() {
        let run_js_script = format!(
            "req={};{}",
            serde_json::to_string(&req_info).unwrap(),
            router_node.header_script
        );
        println!("run_js_script.header => {}\n", &run_js_script);
        let script_data = js::run_js(run_js_script);
        println!("run_js_script.header.data => {}\n", script_data);
        if !script_data.is_null() {
            if script_data.is_object() {
                for (k, v) in script_data.as_object().unwrap() {
                    let value = v.as_str().unwrap();
                    resp.headers.insert(
                        HeaderName::from_lowercase(k.to_lowercase().as_bytes()).unwrap(),
                        value.parse().unwrap(),
                    );
                }
            } else {
                resp.status_code(StatusCode::NOT_FOUND);
                return "header_script is err".to_string();
            }
        }
    }

    println!("router_node = {:?}\n", router_node);
    if router_node.is_html {
        {
            let template;
            let manger = MANGER.get().unwrap().lock().await;
            template = manger.get_template(&router_node.name).unwrap();
            if let Some(data) = &run_js_data {
                let res = template
                    .templage_html
                    .replace("TEMPLAGE_SCRIPT", &serde_json::to_string(data).unwrap());
                println!("res = {}\n", &res);
                res
            } else {
                println!("res = null\n");
                template.templage_html.replace("TEMPLAGE_SCRIPT", "")
            }
        }
    } else {
        if let Some(data) = &run_js_data {
            format!("{}", serde_json::to_string(data).unwrap())
        } else {
            "".to_string()
        }
    }
}

static MANGER: OnceCell<Mutex<Manger>> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    MANGER.set(Mutex::new(router::Manger::new().await)).unwrap();
    let router = Router::new()
        .push(
            Router::with_path("/control")
                .push(
                    Router::with_path("/api")
                        .get(get_api)
                        .post(set_api)
                        .delete(del_api)
                        .put(set_api),
                )
                .push(
                    Router::with_path("/html")
                        .get(get_html)
                        .post(set_html)
                        .delete(del_api)
                        .put(set_html),
                )
                .push(
                    Router::with_path("/template")
                        .get(get_templage)
                        .post(set_templage)
                        .delete(del_templage)
                        .put(set_templage),
                ),
        )
        .push(
            Router::with_path("<**path>").get(
                StaticDir::new(["static"])
                    .defaults("index.html")
                    .auto_list(true),
            ),
        )
        .push(Router::with_path("<**pahts>").goal(manger_run));
    let acceptor = TcpListener::new("127.0.0.1:8080").bind().await;
    let server = Server::new(acceptor);
    server.serve(router).await;
}

#[handler]
async fn get_api() -> String {
    {
        let manger = MANGER.get().unwrap().lock().await;
        let test = manger.get_api(false);
        format!("{}", serde_json::to_string(&test).unwrap())
    }
}
#[handler]
async fn set_api(req: &mut Request) {
    let mut router_node: db::RouterNode = req.parse_json().await.unwrap();
    {
        let mut manger = MANGER.get().unwrap().lock().await;
        router_node.is_html = false;
        manger.set_api(router_node).await;
    }
}
#[handler]
async fn del_api(req: &mut Request) {
    let router_node: db::RouterNode = req.parse_json().await.unwrap();
    {
        let mut manger = MANGER.get().unwrap().lock().await;
        manger.del_api(router_node).await;
    }
}
#[handler]
async fn get_html() -> String {
    {
        let manger = MANGER.get().unwrap().lock().await;
        let test = manger.get_api(true);
        format!("{}", serde_json::to_string(&test).unwrap())
    }
}
#[handler]
async fn set_html(req: &mut Request) {
    let mut router_node: db::RouterNode = req.parse_json().await.unwrap();
    {
        let mut manger = MANGER.get().unwrap().lock().await;
        router_node.is_html = true;
        manger.set_api(router_node).await;
    }
}
#[handler]
async fn get_templage() -> String {
    {
        let manger = MANGER.get().unwrap().lock().await;
        let test = manger.get_templates();
        format!("{}", serde_json::to_string(&test).unwrap())
    }
}
#[handler]
async fn set_templage(req: &mut Request) {
    let template: db::Template = req.parse_json().await.unwrap();
    {
        let mut manger = MANGER.get().unwrap().lock().await;
        manger.set_template(template).await;
    }
}

#[handler]
async fn del_templage(req: &mut Request) {
    let template: db::Template = req.parse_json().await.unwrap();
    {
        let mut manger = MANGER.get().unwrap().lock().await;
        manger.del_template(template).await;
    }
}
