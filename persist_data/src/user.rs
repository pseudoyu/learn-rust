use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Gender {
    Unspecified = 0,
    Male = 1,
    Female = 2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub name: String,
    age: u8,
    pub(crate) gender: Gender,
}

impl User {
    pub fn new(name: String, age: u8, gender: Gender) -> Self {
        Self { name, age, gender }
    }

    pub fn load(filename: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(filename)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let user = serde_json::from_str(&data)?;

        Ok(user)
    }

    pub fn persist(&self, filename: &str) -> Result<usize, std::io::Error> {
        let mut file = File::create(filename)?;

        // match File::create(filename) {
        //     Ok(f) => {
        //         todo!()
        //     }
        //     Err(e) => return Err(e),
        // }

        let data = serde_json::to_string(self)?;
        file.write_all(data.as_bytes())?;
        Ok(data.len())
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            name: "Unknown User".into(),
            age: 0,
            gender: Gender::Unspecified,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let user = User::default();
        let path = "/Users/pseudoyu/Developer/personal/learn-rust/persist_data/user.json";
        user.persist(path).unwrap();

        let user1 = User::load(path).unwrap();
        assert_eq!(user, user1);
    }
}
