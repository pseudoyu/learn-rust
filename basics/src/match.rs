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
