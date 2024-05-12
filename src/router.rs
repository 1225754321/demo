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
        Manger { router_tree, db }
    }
    pub async fn del_template(&mut self, template: Template) {
        self.db.remove_template(&template.name).await;
    }
    pub async fn set_template(&mut self, template: Template) {
        self.db.set_template(template).await;
    }
    pub async fn del_api(&mut self, node: RouterNode) {
        self.db.remove_api(&node.key()).await;
    }
    pub async fn set_api(&mut self, node: RouterNode) {
        self.router_tree
            .store(node.method.clone(), node.path.clone());
        self.db.set_api(node.clone()).await;
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
