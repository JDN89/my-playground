fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(" the area of the rectangle is : {}", area(width1, height1));

    //===== tuple example
    let rect = (30, 50);
    println!("the area of rectangle is: {}", tuple_area(rect));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// tuple fun
fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
