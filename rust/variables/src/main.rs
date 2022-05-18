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
}
