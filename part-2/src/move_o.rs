

pub fn move_o() {

    // ownership = only one owner at a time
    // dangling problam solving

    let a = String::from("hello world");
    let b = a;
    println!("{}", b);
}