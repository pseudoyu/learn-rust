use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<RefCell<Node>>>,
    // 多线程环境下的可变性
    // downstream: Option<Arc<Mutex<Node>>>,
    // downstream: Option<Arc<RwLock<Node>>>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    fn update_downstream(&mut self, downstream: Rc<RefCell<Node>>) {
        self.downstream = Some(downstream);
    }

    fn get_downstream(&self) -> Option<Rc<RefCell<Node>>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

pub fn run() {
    // DAG 实现
    println!("\n>>> DAG start...");
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);

    node3.update_downstream(Rc::new(RefCell::new(node4)));
    node1.update_downstream(Rc::new(RefCell::new(node3)));
    node2.update_downstream(node1.get_downstream().unwrap());

    println!("node1: {:?}\nnode2: {:?}", node1, node2);

    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();

    node3.borrow_mut().downstream = Some(Rc::new(RefCell::new(node5)));


    // 内部可变性
    let data = RefCell::new(1);
    {
        // 获取 RefCell 的内部数据可变借用
        let mut v = data.borrow_mut();
        *v += 1;
    }
    println!("\n>>>Borrow_mut Start...");
    println!("data: {}", data.borrow());
}