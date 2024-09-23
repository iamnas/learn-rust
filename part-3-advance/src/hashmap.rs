use std::collections::HashMap;

pub fn main() {
    let mut users: HashMap<String, u32> = HashMap::new();

    users.insert(String::from("kop"), 23);
    users.insert(String::from("kossp"), 23);
    users.insert(String::from("rama"), 23);

    println!("{:?}",users);
    println!("{:?}",users);

    match users.get("kop") {
        Some(a) => println!("{:?}",a),
        None => println!("dgdgd"),
    }
}
