use std::any::Any;

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
}
