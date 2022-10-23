pub fn run() {
    // 变量与函数
    // 默认不可变
    let name = "Yu";
    println!("My name is {}", name);

    // 通过 mut 关键字声明可变变量
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    println!("v: {:?}", v);

    // 常量，全局可访问
    const CONST_PI: f64 = 3.1415926;
    println!("const pi: {}", CONST_PI);

    // 静态变量，也全局可访问，可声明可变
    static STATIC_PI: f64 = 3.1415926;
    println!("static pi: {}", STATIC_PI);

    static V: Vec<u8> = Vec::new();
    println!("static v: {:?}", V);
}