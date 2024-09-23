// Write a function to read a file 

use std::fs::read_to_string;

pub fn read_file() {

    let file = read_to_string("a.txt");
    match file {
        Ok(data)=> println!("{}", data),
        Err(_err) =>  println!("Error while reading the file this ")
    }
}