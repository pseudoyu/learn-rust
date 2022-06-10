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
