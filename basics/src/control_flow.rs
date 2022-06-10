// loop 循环
fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    println!("fib_loop start...");
    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next fibonacci number: {}", b);

        // 条件分支
        if i >= n {
            // 退出循环
            break;
        }
    }
}

// while 循环

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2u8);

    println!("fib_while start...");
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next fibonacci number: {}", b);
    }
}

// for 循环
// 可以用于任何实现了 IntoIterator trait 的数据结构
fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    println!("fib_for start...");

    // 2..n 是 Range 操作，包含 2 <= x < n 的所有值
    // 不支持负数
    // 可以省略上标/下标
    // arr[..]
    // arr[0..=1]
    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next fibonacci number: {}", b);
    }
}
