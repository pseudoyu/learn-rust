// 使用 include 宏来包含其他文件
include!("data_structure.rs");
include!("match.rs");

// 使用 mod 来包含其他文件
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

    // 函数
    println!("\nFunction start...");
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

    // 控制流
    let n = 10;
    println!("\nControl flow start...");
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

    println!("\nData structure start...");
    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );

    // 模式匹配
    println!("\nMatch start...");
    process_event(&event1);
    process_message(&event3);
}
