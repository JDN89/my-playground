// we use return keyowrd only to exit a function prematurely
// a function body that ends with ; doesn't return a value
// a body that doens't end with ; returns a value
// we us  or omit ; to indicate wether or not a function returns a value

fn main() {
    another_function(5);
    let y = expression();
    println!("value of y is: {}", y)
}
fn another_function(x: i32) {
    println!("the value of x is: {}", x)
}

//statements don;t return a value and last line ends with ;
//expressions return a value
fn expression() -> i32 {
    //function return 5 so no ; at the end
    5
}
