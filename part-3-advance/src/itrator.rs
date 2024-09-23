

pub fn main() {
     let vec = vec![1,2,3,4,5,6,7,8];

     let iter_vec = vec.iter();

     let odd = iter_vec.filter(|x| *x % 2 == 1).map(|x| x * 2);

    //  println!("{:?}",odd);

    let vec2 :Vec<i32>= odd.collect();

    // let mut vec2 = Vec::new();

    //  for x in odd {
    //     // println!("{}",x);
    //     vec2.push(x);
    //  } 

     println!("{:?}",vec2)

}