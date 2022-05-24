#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(" the area of the rectangle is : {}", area(width1, height1));

    //===== tuple example
    let rect = (30, 50);
    println!("the area of rectangle is: {}", tuple_area(rect));

    //============ struct example
    // for when you need to lable your data
    // otherwise just use a tuple

    // ===== print instance of struct var
    // printing a struct is difficutl, becuase the macro doesn't know how we want to display the struct
    // Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown?Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown?
    let rect_struct = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect struct is {:?}", rect_struct);
    // add # for a nicer print format
    println!("rect struct is {:#?}", rect_struct);
    // ref because we want to borrow the struct to calculate it;s area instead of taking over the ownership of it

    println!(
        "the area of sturct rectangle is {}",
        struct_area(&rect_struct)
    );

    // =========== example with dbg macro
    let scale = 2;
    let rect_dbg = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect_dbg);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// tuple fun
fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn struct_area(rect_struct: &Rectangle) -> u32 {
    rect_struct.height * rect_struct.width
}
