
struct User{

    first_name: String,
    last_name: String,
    age:i32

}


pub fn get_user(){
    let u =  User{first_name: String::from("John"), last_name: String::from("John"), age:22};
    println!("{}",u.age);
    println!("{}",u.first_name);
    println!("{}",u.last_name);
}