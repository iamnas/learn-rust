mod fib;
mod get_string_length;
mod is_even;
mod rect_struct;
mod struct_example;

mod cal_enum;
mod option;
mod result;

mod move_o;
mod borrowing;
mod vactor;

use chrono::Utc;


fn main() {
    println!("{}", is_even::is_even(20));
    println!("{}", fib::fib(2));

    let my_string: String = String::from("SSSSS");

    println!("{}", get_string_length::get_string_length(&my_string));
    struct_example::get_user();

    let d = rect_struct::Rect {
        width: 10,
        height: 11,
    };

    println!("area of a rect is {}", d.area());

    let rec_area = cal_enum::Shape::Rectangle(20.0, 22.0);
    println!("area of rec {}", cal_enum::calculate_area(rec_area));

    // Option
    // println!("{:?}",option::find_first_a(String::from("kn[pa")));

    match option::find_first_a(String::from("kn[pa")) {
        Some(value) => println!("{}", value),
        None => println!("a not found"),
    }


    // Result 
    result::read_file();

    println!("{}",Utc::now());



    move_o::move_o();
    // move ownership 

    borrowing::borrowing();

    vactor::ini_vector();
}
