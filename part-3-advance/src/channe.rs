use std::{sync::mpsc, thread};

pub fn main() {
    
    let (tx,rx) = mpsc::channel();

    {
        let v = String::from("hello world");

       thread::spawn(move || {
        tx.send(v).unwrap();
        });

    }

    println!("{}",rx.recv().unwrap());
}
