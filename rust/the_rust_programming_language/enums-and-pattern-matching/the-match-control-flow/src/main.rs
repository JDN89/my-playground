// enum {
// variant(value)
//}

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
}
