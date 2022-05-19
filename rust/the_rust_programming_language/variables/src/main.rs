use std::io;

fn main() {
    let mut x = 5;
    println!("the value is: {}", x);
    x = 6;
    println!("the value is: {}", x);
    // shadowing: declare new value with the same name as the previous value
    // let x = 5 + 2;
    // code is read from top to bottom
    // inner scope uses the first value of x it encounter -> 6
    // because the shadowed value appears after the outer scope
    {
        let x = x * 2;
        println!("the value of x in the inner scope is: {}", x);
    }

    let x = 5 + 2;
    println!("the value of x in the outer scope is: {}", x);

    // tuple and array -> compound types : group multiple valuess together into one typee

    let tup = (500, 4.1, 1);
    let (d, y, z) = tup;
    println!("the value of d is: {}", d);
    println!("the value of tup y is: {}", tup.1);

    let t: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = t.0;

    let six_point_four = t.1;

    let one = t.2;

    //array -> more usefull if you want your data to be stored on the stack instead of the heap
    // elements inside array all off the same type
    // fixed size -> if you want your data to grow or shrink -> use vectore

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // initialize array -> 3 is the value of the array , 5 is the size
    // let a = [3, 3, 3, 3, 3]; :w
    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
