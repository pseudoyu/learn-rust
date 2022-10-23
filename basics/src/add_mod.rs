// 定义 mod
mod add_mod {
    // 定义函数
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

pub fn run() {
    println!("\n>>> Add mod start...");
    println!("add mod test: {}", add_mod::add(1, 2));
}