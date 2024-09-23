

pub fn main() {

    let mut vec =  Vec::new();  

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);
    vec.push(7);

    // vec = filter_even(vec);

    println!("{:?}",filter_even(vec))

}



pub fn filter_even(vec: Vec<u32>) -> Vec<u32> {

    let mut new_vec = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }

    return new_vec;

}