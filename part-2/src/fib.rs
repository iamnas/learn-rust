pub fn fib(num: u32) -> u32 {
    let mut a: u32 = 0;
    let mut b: u32 = 1;

    if num == 0 {
        return a;
    }
    if num == 1 {
        return b;
    }
    // let mut c: u32 = 0;
    // return

    for _ in 1..num {
        let c= b;
        b=b+a;
        a=c;
        // print!("{}",i)
    }

    return b;
}
