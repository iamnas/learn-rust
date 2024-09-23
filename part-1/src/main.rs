// fn main() {
//     // variable type integer  fi32 also there for float
//     let ddd: i32 = 1253;
//     println!("x : {}", ddd);

//     // varibale type boolean

//     let is_male = true;
//     let is_above_18 = true;

//     if is_male {
//         println!("You are male");
//     } else {
//         println!("You are not male");
//     }
//     if is_above_18 {
//         println!("You can vote")
//     } else {
//         println!("You can't vote")
//     }

//     // string type variable

//     let greeting = String::from("Hello, world");
//     println!("{}", greeting);

//     // conditionals loops
//     let x: i32 = 23;
//     let is_even: bool = is_even(x);

//     if is_even {
//         print!("{} is even\n", x);
//     } else {
//         print!("{} is odd\n", x);
//     }

//     let n = 10;

//     for i in 0..n {
//         print!("{}", i);
//     }

//     let ans = sum(20,22);
//     println!("\n{}",ans);
//     let s1 = String::from("hello");
//     let s2 = takes_ownership(s1);
//     println!("{}", s2);

// }

// pub fn is_even(x: i32) -> bool {
//     return x % 2 == 0;
// }

// fn sum(a: i32, b: i32) -> i32 {
//     return a + b;
// }

// fn takes_ownership(some_string: String) -> String {
//     println!("{}", some_string);
//     return some_string; // return the string ownership back to the original main fn
// }

// fn main() {

//     let mut str = String::from("hello");

//     str = take_ownership(str);
//     println!("{}", str);

// }

// fn take_ownership(s: String) -> String {

//     return  s;
// }

// Borrowing and references

// fn main() {
//     let s1 = String::from("hello");

//     let s2 = &s1;

//     println!("{} {}",s1,s2)
// }

// fn main() {
//     feed_user();
//     let mut s1 = String::from("hello");

//     update_str(&mut s1);
//     println!("{}", s1);
// }

// fn update_str(stt: &mut String) {
//     // println!("{}",stt);

//     stt.push_str(" string");
// }

// Struct

// struct User{
//     name : String,
//     age:u32,
// }

// fn feed_user(){
//    let user = User{
//     name:String::from("AAA"),
//     age:52
//    };
//    println!("{}  {}",user.name,user.age)
// }

// implemnet in struct

struct Rec {
    height: u32,
    width: u32,
}

impl Rec {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}


use rand:: {Rng, thread_rng};
use chrono::{Local,Utc};

fn main() {
    let rec = Rec {
        height: 20,
        width: 56,
    };

    println!("area of a rectangle is = {}", rec.area());

    let my_str = String::from("ramans");
    let res = find_first_a(my_str);

    match res {
        Some(i) => println!("The letter 'a' is found in index: {}", i),
        None => println!("The letter 'a' is not fount in the string"),
    }


    let rng = get_rand();

    println!("the random number is {}", rng);


    let now = Utc::now();
    println!("Current date and time in UTC: {}",now);

    let local = Local::now();
    println!("Current date and time in UTC: {}",local);


}

fn find_first_a(s: String) -> Option<i32> {

    for (i,c) in s.chars().enumerate() {
        if c == 'a'
{        return Some(i as i32)}
    }
    return None

}



fn get_rand() -> i32{

    let mut rng = thread_rng();
    return rng.gen();
    // rand
}