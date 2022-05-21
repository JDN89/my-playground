fn main() {
    let s = String::from("Hello world");
    let s1 = &s[..5];
    let s2 = &s[6..];
    println!("{} {}", s1, s2);
    // find first word in s
    let word = find_first_word(&s);
    println!("The first word is {}", word);

    // slice type works also on arrays
    let a = [0, 1, 2, 3, 4];
    //ending_index is one more than the last position in the slice.
    let slice = &a[1..4];
    assert_eq!(slice, [1, 2, 3]);
}

//&str is the string slice type
fn find_first_word(s: &String) -> &str {
    // convert string to an array of bytes
    let bytes = s.as_bytes();
    // enumerate over element
    // enumerate wraps the result of iter and returns each element as part of a tuple instead.
    // The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
