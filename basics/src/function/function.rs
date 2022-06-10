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
