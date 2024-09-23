use std::thread;

pub fn main() {
    let x = 1;

    {
        let v = vec![1, 2, 3, 4, 5];

       let handel =  thread::spawn(move || {
            println!("{:?}", v);
        });

        handel.join().unwrap()
    }

    print!("{}",x)
}
