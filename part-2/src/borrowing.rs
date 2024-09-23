pub fn borrowing() {
    let mut a = String::from("Hello, world!");

    call_fn(&a);

    call_fn_mut(&mut a);

    println!("{}", a);
}

fn call_fn(b: &String) {
    println!("{}", b);
}

// borrowing and mutate

fn call_fn_mut(b: &mut String) {
    b.push_str("string");
    println!("{}", b);
}
