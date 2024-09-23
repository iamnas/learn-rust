

mod vactor;
mod hashmap;
mod itrator;
mod slice;
mod trait_ex;
mod lifetime;

mod thread_example;

mod channe;

fn main() {

    vactor::main();
    hashmap::main();

    itrator::main();

    slice::main();

    trait_ex::main();

    lifetime::main();

    let a = String::from("HELLO");
    let b = String::from("PO");

    longest(a, b);

    thread_example::main();

    channe::main();


}


fn longest(a:String,b:String) -> String {

    if a.len() > b.len() {
        return a;
    }
    else{
        return  b;
    }
}


// write a function that takes two stings as an input and returns the bigger amongst them