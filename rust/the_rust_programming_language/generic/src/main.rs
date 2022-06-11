//create function that takes a list
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    // we return largest -> without ; at the end
    largest
}

fn main() {
    let number_list = vec![2, 5, 11, 6, 44, 33];
    let result = largest(&number_list);
    println!("{}", result);
}
