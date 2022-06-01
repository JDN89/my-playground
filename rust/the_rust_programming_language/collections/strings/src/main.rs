// String wordt in de heap opgeslaan
// $str in de stack
//
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    let s4 = String::from("no world");
    // s1 is gemoved en kan niet langer gebruikt worden
    // ownership van s1 is overgenomen door s3
    // we can only add a &str to a String; we can’t add two String values together
    //looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result
    println!("{}", s3);
    let s = format!("{}-{}-{}", s4, s2, s3);
    let k = &s2[..2];
    println!("{}", s);
    // The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes. For
    // if you want a piece of a string use [0..4]
    println!("{}", k);
}
// je kan strings op 3 verschillende manier bekijken
// bytes // scalar values // graphme clusters -> daardooor is indexen niet mogelijk
// depending on the type you get another index
