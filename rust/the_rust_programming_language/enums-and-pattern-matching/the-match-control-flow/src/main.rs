// enum {
// variant(value)
//}

//Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.
//n Rust prevents us from forgetting to explicitly handle the None case, it protects us from assuming that we have a value when we might have null, thus making the billion-dollar mistake discussed earlier impossible.

enum Address {
    Zonnestraat,
    Eignestraat,
    Poel,
}
fn main() {
    fn wie_woont_waar(address: Address) -> String {
        match address {
            Address::Zonnestraat => String::from("ouders"),
            Address::Eignestraat => String::from("meter"),
            Address::Poel => String::from("Peter"),
        }
    }
    let resutl = wie_woont_waar(Address::Eignestraat);
    println!("{}", resutl);

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
