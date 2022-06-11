// using  std::cmp::PartialOrd
//create function that takes a list

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    // we return largest -> without ; at the end
    largest
}
// example of GENERICS in sstructs
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![2, 5, 11, 6, 44, 33];
    let result = largest(&number_list);
    //CAll genereic STRUCT
    let mix = Point { x: 4, y: 4.5 };

    println!("{}", result);
    println!("{:?}", mix);
    println!("{:?}", mix.x);
    println!("{:?}", mix.y);
}
