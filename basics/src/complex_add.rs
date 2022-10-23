// 实现 Add trait
// trait Add<Rhs = Self> {
//     type Output;
//     #[must_use]
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

use std::ops::Add;

// 定义复数
#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

// 为复数实现 Add trait
impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

// 实现 Add trait，实数与复数相加
impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;
        let imagine = self.imagine;
        Complex::new(real, imagine)
    }
}

pub fn run() {
    // 复数 Add
    println!("\n>>> Complex Add start...");
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);
    println!("output: {:?}", &c1 + &c2);
    println!("output: {:?}", &c1 + 5.0);
    println!("output: {:?}", c1 + c2);
}