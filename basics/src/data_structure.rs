// 数据结构

// enum 定义枚举类型

#[derive(Debug, Copy, Clone)]
enum Gender {
    #[allow(dead_code)]
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

// struct 定义结构体

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug, Clone)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug, Clone)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Event {
    Join(UserId, TopicId),
    Leave(UserId, TopicId),
    Message(UserId, TopicId, String),
}

// 模式匹配
// 可以用于 struct / enum 中匹配部分或者全部内容

fn process_event(event: &Event) {
    match event {
        Event::Join(uid, _tid) => println!("user {:?} joined", uid),
        Event::Leave(uid, tid) => println!("user {:?} left {:?}", uid, tid),
        Event::Message(_, _, msg) => println!("message: {}", msg),
    }
}

// 简单匹配
fn process_message(event: &Event) {
    if let Event::Message(_, _, msg) = event {
        println!("message: {}", msg);
    }
}

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct ColorTuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
    
pub fn run() {
    // Traditional Struct
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // Tuple Struct
    let mut ct = ColorTuple(255, 0, 0);
    ct.0 = 200;
    println!("Color: {} {} {}", ct.0, ct.1, ct.2);

    // Person
    let mut p = Person::new("Yu", "ZHANG");
    println!("Person {} {}", p.first_name, p.last_name);
    p.set_last_name("Zhang");
    println!("Person {}", p.full_name());
    println!("{:?}", p.to_tuple());

    // Example
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

    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );

    // 模式匹配
    process_event(&event1);
    process_message(&event3);
}