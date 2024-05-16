use ::clap::Parser;
use clap::Cli;
use router::Manger;
use salvo::http::header::LOCATION;
use salvo::serve_static::StaticDir;
use salvo::{
    http::{mime, HeaderName},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::{Mutex, OnceCell};
pub mod clap;
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
async fn manger_run(req: &mut Request, resp: &mut Response) {
    if req.uri().path() == "/" {
        resp.headers
            .insert(LOCATION, "/static/api.html".parse().unwrap());
        resp.status_code(StatusCode::FOUND);
        return;
    }
    if req.uri().path() == "/favicon.ico" {
        resp.headers
            .insert(LOCATION, "/static/favicon.ico".parse().unwrap());
        resp.status_code(StatusCode::FOUND);
        return;
    }

    println!("manger_run.uri => {}", req.uri());

    let mut files: Vec<HashMap<String, String>> = Vec::new();
    let mut from_utf8: String = String::new();
    let mut from_json = HashMap::new();

    println!("manger_run.uri2 => {}", req.uri());
    // 获取router_node
    let router_node: db::RouterNode;
    {
        let manger = MANGER.get().unwrap().lock().await;
        println!(
            "manger_run.uri2.1 => {} - {}",
            &req.method().to_string(),
            &req.uri().path().to_string()
        );
        if let Some(temp) = manger.query(
            &req.method().to_string().to_uppercase(),
            &req.uri().path().to_string(),
        ) {
            println!("manger_run.uri2.2 => {}", req.uri());
            router_node = temp.clone();
        } else {
            println!("manger_run.uri2.3 => {}", req.uri());
            resp.status_code(StatusCode::NOT_FOUND);
            return;
        }
    }

    println!("manger_run.uri3 => {}", req.uri());
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

    println!("manger_run.uri4 => {}", req.uri());
    let r = req.payload().await.unwrap();
    if let Ok(temp) = String::from_utf8(r.to_vec()) {
        if !temp.is_empty() {
            from_utf8 = temp;
        }
    }

    println!("manger_run.uri5 => {}", req.uri());
    if let Ok(parse_json) = req.parse_json::<HashMap<String, Value>>().await {
        if !parse_json.is_empty() {
            from_json = parse_json;
        }
    }

    println!("manger_run.uri6 => {}", req.uri());
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

    println!("manger_run.uri7 => {}", req.uri());
    let mut run_js_data = Option::None;
    if !router_node.script.is_empty() && !router_node.is_html {
        let run_js_script = format!(
            "req={};{}",
            serde_json::to_string(&req_info).unwrap(),
            router_node.script
        );
        println!("run_js_script.script => {}\n", &run_js_script);
        run_js_data = Some(js::run_js(&run_js_script));
        println!("run_js_script.script.data => {:?}\n", &run_js_data);
    }

    println!("manger_run.uri8 => {}", req.uri());
    // 设置code
    if !router_node.code_script.is_empty() {
        let run_js_script = format!(
            "req={};{}",
            serde_json::to_string(&req_info).unwrap(),
            router_node.code_script
        );
        println!("run_js_script.code => {}\n", &run_js_script);
        let script_data = js::run_js(&run_js_script);
        println!("run_js_script.code.data => {}\n", script_data);
        let code = script_data.as_i64().unwrap();
        let code = StatusCode::from_u16(code as u16).unwrap();
        resp.status_code(code);
    }

    println!("manger_run.uri9 => {}", req.uri());
    // 设置headers
    if !router_node.header_script.is_empty() {
        let run_js_script = format!(
            "req={};{}",
            serde_json::to_string(&req_info).unwrap(),
            router_node.header_script
        );
        println!("run_js_script.header => {}\n", &run_js_script);
        let script_data = js::run_js(&run_js_script);
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
                return;
            }
        }
    }

    println!("router_node = {:?}\n", router_node);
    if router_node.is_html {
        {
            let template;
            let manger = MANGER.get().unwrap().lock().await;
            template = manger.get_template(&router_node.template).unwrap();
            let res = template.templage_html.replace(
                "{ { DATA } }",
                &format!(
                    "req={};{};",
                    &serde_json::to_string(&req_info).unwrap(),
                    router_node.script
                ),
            );
            println!("res = {}\n", &res);
            resp.render(Text::Html(res));
        }
    } else {
        if let Some(data) = &run_js_data {
            resp.render(Text::Json(format!(
                "{}",
                serde_json::to_string(data).unwrap()
            )));
            return;
        } else {
            return;
        }
    }
}

static MANGER: OnceCell<Mutex<Manger>> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    let parse = Cli::parse();
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
                        .put(update_api),
                )
                .push(
                    Router::with_path("/html")
                        .get(get_html)
                        .post(set_html)
                        .delete(del_api)
                        .put(update_url),
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
            Router::with_path("/static/<**path>").get(
                StaticDir::new(["static"])
                    .defaults("api.html")
                    .auto_list(true),
            ),
        )
        .push(Router::with_path("<**pahts>").goal(manger_run));
    let acceptor = TcpListener::new(format!("{}:{}", parse.host.unwrap(), parse.port.unwrap()))
        .bind()
        .await;
    let server = Server::new(acceptor);
    server.serve(router).await;
}

#[derive(Debug, Serialize, Deserialize)]
struct Res<T> {
    status: i64,
    msg: String,
    data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Page<T> {
    total: usize,
    items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PageQ {
    page: Option<usize>,
    #[serde(alias = "perPage")]
    per_page: Option<usize>,
    name: Option<String>,
    path: Option<String>,
}

#[handler]
async fn get_api(req: &mut Request) -> String {
    {
        let page_q: PageQ = req.parse_queries().unwrap();
        println!("get_api => {:?}", page_q);
        let left = (page_q.page.unwrap() - 1) * page_q.per_page.unwrap();
        let manger = MANGER.get().unwrap().lock().await;
        let test = manger.get_api(false);
        let mut right = left + page_q.per_page.unwrap();
        if right > test.len() {
            right = test.len();
        }
        format!(
            "{}",
            serde_json::to_string(&Res {
                status: 0,
                msg: "".to_string(),
                data: Some(Page {
                    total: test.len(),
                    items: test[left..right]
                        .iter()
                        .filter(|v| {
                            v.name
                                .contains(page_q.name.clone().unwrap_or_default().as_str())
                                && v.path
                                    .contains(page_q.path.clone().unwrap_or_default().as_str())
                        })
                        .collect()
                })
            })
            .unwrap()
        )
    }
}
#[handler]
async fn set_api(req: &mut Request) -> String {
    let mut router_node: db::RouterNode = req.parse_json().await.unwrap();
    {
        let mut manger = MANGER.get().unwrap().lock().await;
        router_node.is_html = false;
        manger.set_api(router_node).await;
    }
    format!(
        "{}",
        serde_json::to_string(&Res::<String> {
            status: 0,
            msg: "".to_string(),
            data: None
        })
        .unwrap()
    )
}
#[handler]
async fn update_api(req: &mut Request) -> String {
    let mut router_node: db::RouterNode = req.parse_json().await.unwrap();
    let key: String = req.query("key").unwrap();
    {
        let mut manger = MANGER.get().unwrap().lock().await;
        router_node.is_html = false;
        println!("update_api =>{:?}", router_node);
        manger.del_api(&key).await;
        manger.set_api(router_node).await;
    }
    format!(
        "{}",
        serde_json::to_string(&Res::<String> {
            status: 0,
            msg: "".to_string(),
            data: None
        })
        .unwrap()
    )
}
#[handler]
async fn update_url(req: &mut Request) -> String {
    let mut router_node: db::RouterNode = req.parse_json().await.unwrap();
    let key: String = req.query("key").unwrap();
    {
        let mut manger = MANGER.get().unwrap().lock().await;
        router_node.is_html = true;
        println!("update_url =>{:?}", router_node);
        manger.del_api(&key).await;
        manger.set_api(router_node).await;
    }
    format!(
        "{}",
        serde_json::to_string(&Res::<String> {
            status: 0,
            msg: "".to_string(),
            data: None
        })
        .unwrap()
    )
}
#[handler]
async fn del_api(req: &mut Request) -> String {
    let router_node: db::RouterNode = req.parse_json().await.unwrap();
    {
        let mut manger = MANGER.get().unwrap().lock().await;
        println!("del_api =>{:?}", router_node);
        manger.del_api(&router_node.key()).await;
    }
    format!(
        "{}",
        serde_json::to_string(&Res::<String> {
            status: 0,
            msg: "".to_string(),
            data: None
        })
        .unwrap()
    )
}
#[handler]
async fn get_html(req: &mut Request) -> String {
    {
        let page_q: PageQ = req.parse_queries().unwrap();
        println!("get_html => {:?}", page_q);
        let left = (page_q.page.unwrap() - 1) * page_q.per_page.unwrap();
        let manger = MANGER.get().unwrap().lock().await;
        let test = manger.get_api(true);
        let mut right = left + page_q.per_page.unwrap();
        if right > test.len() {
            right = test.len();
        }
        format!(
            "{}",
            serde_json::to_string(&Res {
                status: 0,
                msg: "".to_string(),
                data: Some(Page {
                    total: test.len(),
                    items: test[left..right]
                        .iter()
                        .filter(|v| {
                            v.name
                                .contains(page_q.name.clone().unwrap_or_default().as_str())
                                && v.path
                                    .contains(page_q.path.clone().unwrap_or_default().as_str())
                        })
                        .collect()
                })
            })
            .unwrap()
        )
    }
}
#[handler]
async fn set_html(req: &mut Request) -> String {
    let mut router_node: db::RouterNode = req.parse_json().await.unwrap();
    {
        let mut manger = MANGER.get().unwrap().lock().await;
        router_node.is_html = true;
        manger.set_api(router_node).await;
    }
    format!(
        "{}",
        serde_json::to_string(&Res::<String> {
            status: 0,
            msg: "".to_string(),
            data: None
        })
        .unwrap()
    )
}
#[handler]
async fn get_templage(req: &mut Request) -> String {
    {
        let page_q: PageQ = req.parse_queries().unwrap();
        println!("get_templage => {:?}", page_q);
        let manger = MANGER.get().unwrap().lock().await;
        let test = manger.get_templates();
        let left;
        let mut right;
        if page_q.page.is_none() && page_q.per_page.is_none() {
            left = 0;
            right = test.len();
        } else {
            left = (page_q.page.unwrap() - 1) * page_q.per_page.unwrap();
            right = left + page_q.per_page.unwrap();
            if right > test.len() {
                right = test.len();
            }
        }

        format!(
            "{}",
            serde_json::to_string(&Res {
                status: 0,
                msg: "".to_string(),
                data: Some(Page {
                    total: test.len(),
                    items: test[left..right]
                        .iter()
                        .filter(|v| {
                            v.name
                                .contains(page_q.name.clone().unwrap_or_default().as_str())
                        })
                        .collect()
                })
            })
            .unwrap()
        )
    }
}
#[handler]
async fn set_templage(req: &mut Request) -> String {
    let template: db::Template = req.parse_json().await.unwrap();
    {
        let mut manger = MANGER.get().unwrap().lock().await;
        manger.set_template(template).await;
    }
    format!(
        "{}",
        serde_json::to_string(&Res::<String> {
            status: 0,
            msg: "".to_string(),
            data: None
        })
        .unwrap()
    )
}

#[handler]
async fn del_templage(req: &mut Request) -> String {
    let template: db::Template = req.parse_json().await.unwrap();
    {
        let mut manger = MANGER.get().unwrap().lock().await;
        manger.del_template(template).await;
    }
    format!(
        "{}",
        serde_json::to_string(&Res::<String> {
            status: 0,
            msg: "".to_string(),
            data: None
        })
        .unwrap()
    )
}
