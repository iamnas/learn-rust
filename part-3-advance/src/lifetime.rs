struct User<'a> {
    name: &'a str,
}

pub fn main(){
    let name = String::from("apple");

    let user = User{
        name: &name,
    };
    println!("{}",user.name);
}