struct User {
    email: String,
    active: bool,
    username: String,
    sign_in_count: u32,
}
//tuple struct
//  Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples
// and when naming each field as in a regular struct would be verbose or redundant.

struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("jan@deniels.com"),
        username: String::from("jdn"),
        active: true,
        sign_in_count: 1,
    };
    println!("username is: {}", user1.username);
    let user2 = User {
        email: String::from("another@mail.com"),
        ..user1 // .. indicates that the remaining fields stay the same
    };
    println!("email user 2: {}", user2.email);
    println!("email user 1: {}", user1.email);

    // tuple struct
    let black = Color(0, 0, 0);

    // prin_user function causes error because we moved the value email to user 2
    // email is a string and so the ownership of the email field
    // has been moved to user2
    print_user(user1);
}

fn print_user(
    User {
        email,
        active,
        username,
        sign_in_count,
    }: User,
) {
    println!("{} is valid?", email);
}
