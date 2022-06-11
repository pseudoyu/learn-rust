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
