use regex::Regex;
use std::str::FromStr;

trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

// impl Parse for u8 {
//     fn parse(s: &str) -> Self {
//         let re: Regex = Regex::new(r"^[0-9]+").unwrap();
//         if let Some(captures) = re.captures(s) {
//             captures
//                 .get(0)
//                 .map_or(0, |s| s.as_str().parse().unwrap_or(0))
//         } else {
//             0
//         }
//     }
// }

// 使用泛型实现
impl<T> Parse for T
where
    T: FromStr + Default,
{
    type Error = String;
    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(Err("failed to capture".to_string()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_err| "failed to capture".to_string())
                })
        } else {
            Err("failed to capture".to_string())
        }
    }
}

// 子类型多态
struct Cat;
struct Dog;

trait Animal {
    fn get_name(&self) -> &'static str;
}

fn get_name(animal: impl Animal) -> &'static str {
    animal.get_name()
}

impl Animal for Cat {
    fn get_name(&self) -> &'static str {
        "Cat"
    }
}

impl Animal for Dog {
    fn get_name(&self) -> &'static str {
        "Dog"
    }
}

pub fn run() {
    // 实现 trait
    println!("\n>>> Trait start...");
    println!("result: {:?}", u8::parse("255 hello world"));
    
    // 子类型多态
    println!("\n>>> Subtype polymorphism start...");
    let cat = Cat;
    println!("cat: {}", get_name(cat));
}

// 测试，使用 `cargo test` 执行
#[test]
fn parse_test() {
    assert_eq!(u32::parse("123abcd"), Ok(123));
    assert_eq!(
        u32::parse("123.45abcd"),
        Err("failed to capture".to_string().into())
    );
    assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
    assert!(f64::parse("abcd").is_err());
}