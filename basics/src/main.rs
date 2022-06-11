// 使用 include 宏来包含其他文件
include!("data_structure.rs");
include!("match.rs");
include!("dag.rs");
include!("life_cycle.rs");
include!("trait.rs");
include!("complex_add.rs");

// 定义 mod
mod add_mod {
    // 定义函数
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// 添加单元测试
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// 包含其他文件为 mod
mod control_flow;
mod function;

fn main() {
    // 变量与函数
    // 默认不可变
    let name = "Yu";

    // 通过 mut 关键字声明可变变量
    let mut v: Vec<u8> = Vec::new();

    // 常量，全局可访问
    const CONST_PI: f64 = 3.1415926;

    // 静态变量，也全局可访问，可声明可变
    static static_p: f64 = 3.1415926;
    static V: Vec<u8> = Vec::new();

    // 模块
    println!("\n>>> Add mod start...");
    println!("add mod test: {}", add_mod::add(1, 2));

    // 函数
    println!("\n>>> Function start...");
    println!("apply square: {}", function::apply(2, function::square));
    println!("apply cube: {}", function::apply(2, function::cube));

    // 函数类型与返回值
    let is_pi = function::pi();
    let is_unit_1 = function::not_pi();
    let is_unit_2 = {
        function::pi();
    };

    println!(
        "is_pi: {:?}, is_unit_1: {:?}, is_unit_2: {:?}",
        is_pi, is_unit_1, is_unit_2
    );

    // 查找数组位置
    println!("\n>>> Find pos start...");
    let data = vec![10, 42, 9, 8];
    let v = 42;
    if let Some(pos) = function::find_pos(data, v) {
        println!("{} is at position {}", v, pos);
    }

    // 数组求和
    let data1 = vec![1, 2, 3, 4];

    // 使用 clone 方法复制一个新的数组
    let data2 = data1.clone();

    // 通过借用访问数组
    let data3 = &data1;

    println!("\n>>> Vec sum start...");
    println!("sum of data 2: {}", function::sum(data2.clone()));
    println!("data2: {:?}", data2);
    println!("sum of data1: {}", function::sum(data1));

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

    // 生命周期
    println!("\n>>> Life Cycle start...");
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);

    // 控制流
    let n = 10;
    println!("\n>>> Control flow start...");
    control_flow::fib_loop(n);
    control_flow::fib_while(n);
    control_flow::fib_for(n);

    // 数据结构
    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };

    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Gender::Male,
    };

    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(),
        owner: UserId(1),
    };

    let event1 = Event::Join(alice.id, topic.id);
    let event2 = Event::Join(bob.id, topic.id);
    let event3 = Event::Message(alice.id, topic.id, "Hello, world!".into());

    println!("\n>>> Data structure start...");
    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );

    // 模式匹配
    println!("\n>>> Match start...");
    process_event(&event1);
    process_message(&event3);

    // 实现 trait
    println!("\n>>> Trait start...");
    println!("result: {:?}", u8::parse("255 hello world"));

    // 复数 Add
    println!("\n>>> Complex Add start...");
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);
    println!("output: {:?}", &c1 + &c2);
    println!("output: {:?}", c1 + c2);
}

// 测试，使用 `cargo test` 执行
#[test]
fn parse_test() {
    assert_eq!(u32::parse("123abcd"), Ok(123));
    assert_eq!(
        u32::parse("123.45abcd"),
        Err("failed to capture".to_string().into())
    );
    assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
    assert!(f64::parse("abcd").is_err());
}
