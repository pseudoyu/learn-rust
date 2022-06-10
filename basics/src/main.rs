// 变量与函数

// 函数可以作为参数或者返回值
fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

// 如果最后一个表达式没有以 ; 结尾，那么最后一个表达式就是它的返回值，如果在函数中需要提前返回则使用 return
fn pi() -> f64 {
    3.1415926
}

// 如果以 ; 结尾则返回 unit
fn not_pi() {
    3.1415926;
}

fn main() {
    // 变量与函数
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
}
