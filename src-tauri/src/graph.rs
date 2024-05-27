use rand::Rng;
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};
pub struct NodeVec {
    pub x: f64,
    pub y: f64,
}
pub struct DataNode {
    pub id: usize,
    pub size: usize,
    pub pos: NodeVec,
    edges: Vec<usize>,
}

impl fmt::Debug for DataNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ id:{},size:{},x:{},y:{} }}",
            self.id, self.size, self.pos.x, self.pos.y
        )
    }
}

impl DataNode {
    fn new(id: usize, size: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            size: 1,
            pos: NodeVec {
                x: rng.gen_range(0.0..size),
                y: rng.gen_range(0.0..size),
            },
            edges: Vec::new(),
        }
    }

    fn connect(node: Rc<RefCell<DataNode>>, other: Rc<RefCell<DataNode>>) {
        node.borrow_mut().edges.push(other.borrow().id);
        other.borrow_mut().edges.push(node.borrow().id);
    }

    fn has_edge(&self, other: &DataNode) -> bool {
        self.edges.iter().any(|&edge| edge == other.id)
    }
}

pub struct DataBuild {
    spring_length: f64,
    spring_strength: f64,
    repulsion_strength: f64,
    skip_distance: f64,
    max_second: usize,
    size: f64,
    step: usize,
}

pub trait Data {
    fn id(&self) -> usize;
}
pub trait Link {
    fn source(&self) -> usize;
    fn target(&self) -> usize;
}

impl DataBuild {
    pub fn default() -> Self {
        Self {
            spring_length: 500.0,
            spring_strength: 0.0001,
            repulsion_strength: 1500.0,
            skip_distance: 575.0,
            max_second: 20000,
            step: 1,
            size: 2000.0,
        }
    }

    pub fn builds(
        &self,
        datas: Vec<Box<dyn Data>>,
        links: Vec<Box<dyn Link>>,
    ) -> Vec<Rc<RefCell<DataNode>>> {
        let mut data_nodes = Vec::new();
        let mut m_data = HashMap::new();

        for data in datas {
            let node = Rc::new(RefCell::new(DataNode::new(data.id(), self.size)));
            m_data.insert(data.id(), node.clone());
            data_nodes.push(node);
        }

        for link in links {
            if link.source() == link.target() {
                panic!("x {} = y {}", link.source(), link.target());
            }
            DataNode::connect(
                m_data[&link.source()].clone(),
                m_data[&link.target()].clone(),
            );
            DataNode::connect(
                m_data[&link.target()].clone(),
                m_data[&link.source()].clone(),
            );
            m_data[&link.source()].borrow_mut().size += self.step;
            m_data[&link.target()].borrow_mut().size += self.step;
        }

        for _ in 0..self.max_second {
            self.force_directed_no_repeat_skip_no_vector(&mut data_nodes);
        }

        data_nodes
    }

    fn force_directed_no_repeat_skip_no_vector(&self, graph: &mut Vec<Rc<RefCell<DataNode>>>) {
        for i in 0..graph.len() {
            let mut node = graph[i].borrow_mut();
            for j in (i + 1)..graph.len() {
                let mut other = graph[j].borrow_mut();

                let apart_x = other.pos.x - node.pos.x;
                let apart_y = other.pos.y - node.pos.y;
                let distance = (apart_x * apart_x + apart_y * apart_y).sqrt().max(1.0);

                let has_edge = node.has_edge(&other);
                if !has_edge && distance > self.skip_distance {
                    continue;
                }
                let mut force_size = -self.repulsion_strength / (distance * distance);
                if has_edge {
                    force_size += (distance - self.spring_length) * self.spring_strength;
                }

                let force_x = apart_x * force_size / distance;
                let force_y = apart_y * force_size / distance;

                node.pos.x += force_x;
                node.pos.y += force_y;

                other.pos.x -= force_x;
                other.pos.y -= force_y;
            }
        }
    }
}
//下面为一个测试模块，里面的测试函数it_works对上面的add函数进行测试
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[derive(Debug)]
    struct NodeData {
        id: usize,
    }
    impl Data for NodeData {
        fn id(&self) -> usize {
            self.id
        }
    }

    struct DataLink {
        source: usize,
        target: usize,
    }
    impl Link for DataLink {
        fn source(&self) -> usize {
            self.source
        }

        fn target(&self) -> usize {
            self.target
        }
    }

    #[test]
    #[ignore]
    fn graph_test() {
        let mut rng = rand::thread_rng();
        let mut datas: Vec<Box<dyn Data>> = Vec::new();
        let mut links: Vec<Box<dyn Link>> = Vec::new();
        for id in 0..100 {
            datas.push(Box::new(NodeData { id }));
            for _ in 0..rng.gen_range(0..10) as usize {
                let target = rng.gen_range(0..100);
                if id != target {
                    links.push(Box::new(DataLink { source: id, target }))
                }
            }
        }
        let builds = DataBuild::default().builds(datas, links);
        println!("builds => {:?}", builds)
    }
}
