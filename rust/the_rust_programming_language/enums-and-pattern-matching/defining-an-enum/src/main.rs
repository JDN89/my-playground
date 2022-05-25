// the name of each enum variant that we define also becomes a function that constructs an instance of the enum
//That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// just like with structs, we can define methods on enums
impl IpAddr {
    fn print(&self) {
        println!("printed val self {:?}", &self);
    }
}

fn main() {
    let home = IpAddr::V4(123, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("loopback adres is: {:?}", loopback);
    IpAddr::print(&home);
}
