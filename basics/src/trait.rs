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
