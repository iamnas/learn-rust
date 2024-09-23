trait Summery {
    fn summery(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summery for User {
    fn summery(&self) -> String {
        self.name.clone()
    }
}

pub fn main() {}
