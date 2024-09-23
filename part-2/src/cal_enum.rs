pub enum Shape {
    Rectangle(f64, f64),
    Cricle(f64),
}

pub fn calculate_area(shape: Shape) -> f64 {

    match shape {
        Shape::Cricle(a) => a*a*3.14,
        Shape::Rectangle(a,b) => a*b
    }
}



// pub fn calculate_area_2(shape: Shape) -> f64 {

//     let area = match shape {
//         Shape::Cricle(a) => a*a*3.14,
//         Shape::Rectangle(a,b) => a*b
//     };
//     return area;
// }