use crate::db::*;
use crate::tree::Pretree;
#[derive(Debug)]
pub struct Manger {
    router_tree: Pretree,
    db: DB,
}

impl Manger {
    pub async fn new() -> Manger {
        let router_tree = Pretree::new();
        let db = DB::new_default().await;
        let mut m = Manger {
            router_tree,
            db: db.clone(),
        };
        for r in db.datas.api_routers.values() {
            m.set_api(r.clone()).await;
        }
        return m;
    }
    pub async fn del_template(&mut self, template: Template) {
        self.db.remove_template(&template.name).await;
    }
    pub async fn set_template(&mut self, template: Template) {
        self.db.set_template(template).await;
    }
    pub fn get_template(&self, name: &str) -> Option<&Template> {
        self.db.datas.templates.get(name)
    }
    pub fn get_templates(&self) -> Vec<&Template> {
        self.db.datas.templates.values().into_iter().collect()
    }
    pub async fn del_api(&mut self, key: &str) {
        self.db.remove_api(key).await;
    }
    pub async fn set_api(&mut self, mut node: RouterNode) {
        node.method = node.method.to_uppercase();
        self.router_tree
            .store(node.method.clone(), node.path.clone());
        self.db.set_api(node.clone()).await;
    }
    pub fn get_api(&self, is_html: bool) -> Vec<&RouterNode> {
        self.db
            .datas
            .api_routers
            .values()
            .into_iter()
            .filter(|ref v| v.is_html == is_html)
            .collect()
    }
    pub fn query(&self, method: &str, url_path: &str) -> Option<&RouterNode> {
        let node = self.router_tree.query(method, url_path)?;
        self.db
            .datas
            .api_routers
            .get(&format!("{}:{}", method, node.rule()))
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::Pretree;
    #[test]
    fn it_works() {
        // 创建一个 Trie 数据结构
        let mut trie: Pretree = Pretree::new();

        // 添加一些路由路径和对应的处理函数
        trie.store("GET".into(), "/users".into());
        trie.store("GET".into(), "/users/:id".into());
        trie.store("GET".into(), "/posts".into());
        trie.store("GET".into(), "/posts/:id".into());

        // 匹配一个路由路径
        let n = trie.query("GET", "account/929239");
        println!("n: {:?} ", n);
        let n = trie.query("GET", "/users/929239");
        println!("n: {:?} ", n);
    }
}
