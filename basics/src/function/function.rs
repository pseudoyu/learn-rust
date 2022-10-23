// 函数可以作为参数或者返回值
pub fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

pub fn square(value: i32) -> i32 {
    value * value
}

pub fn cube(value: i32) -> i32 {
    value * value * value
}

// 如果最后一个表达式没有以 ; 结尾，那么最后一个表达式就是它的返回值，如果在函数中需要提前返回则使用 return
pub fn pi() -> f64 {
    3.1415926
}

// 如果以 ; 结尾则返回 unit
pub fn not_pi() {
    3.1415926;
}

pub fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }
    None
}

pub fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}

pub fn run() {
    // 函数
    println!("\n>>> Function start...");
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

    // 查找数组位置
    println!("\n>>> Find pos start...");
    let data = vec![10, 42, 9, 8];
    let v = 42;
    if let Some(pos) = find_pos(data, v) {
        println!("{} is at position {}", v, pos);
    }

    // 数组求和
    let data1 = vec![1, 2, 3, 4];

    // 使用 clone 方法复制一个新的数组
    let data2 = data1.clone();

    // 通过借用访问数组
    let data3 = &data1;

    println!("\n>>> Vec sum start...");
    println!("sum of data 2: {}", sum(data2.clone()));
    println!("data2: {:?}", data2);
    println!("sum of data1: {}", sum(data1));
}