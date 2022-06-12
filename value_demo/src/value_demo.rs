use std::{collections::HashMap, thread};

pub fn value_demo() {
    let authenticated = true;

    // if authenticated {
    //     println!("User is authenticated.");
    //     todo!()
    // } else {
    //     println!("User is not authenticated.");
    //     tode!()
    // }

    // modify value
    let mut total = 0;
    total += 1;

    // pass to function
    let name = "Arthur".to_string();
    print_my_name(name);

    // pass by ref
    let mut map: HashMap<String, String> = HashMap::new();
    print_map(&map);
    insert_map(&mut map);

    // multithreading
    let mut data = vec![1, 2, 3];

    // thread::spawn(|| {
    //     data.push(5);
    // });

    data.push(4);
    println!("{:?}", data);
}

fn print_my_name(name: String) {
    println!("Hello, {}!", name);
}

fn print_map(map: &HashMap<String, String>) {
    println!("{:?}", map);
}

fn insert_map(map: &mut HashMap<String, String>) {
    map.insert("key".to_string(), "value".to_string());
    println!("{:?}", map);
}
