include!("function.rs");
include!("data_structure.rs");
include!("control_flow.rs");

fn main() {
    // 变量与函数
    // 默认不可变
    let name = "Yu";

    // 通过 mut 关键字声明可变变量
    let mut v: Vec<u8> = Vec::new();

    // 常量，全局可访问
    const PI: f64 = 3.1415926;

    // 静态变量，也全局可访问，可声明可变
    static p: f64 = 3.1415926;
    static V: Vec<u8> = Vec::new();

    // 函数
    println!("Function start...");
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));

    // 函数类型与返回值
    let is_pi = pi();
    let is_unit_1 = not_pi();
    let is_unit_2 = {
        pi();
    };

    println!(
        "is_pi: {:?}, is_unit_1: {:?}, is_unit_2: {:?}",
        is_pi, is_unit_1, is_unit_2
    );

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

    println!("Data structure start...");
    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );

    // 控制流
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}
