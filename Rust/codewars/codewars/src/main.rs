fn get_middle(s: &str) -> &str {
    let len = s.len();
    let from = (len - 1) / 2;

    println!("{:?}", from);
    let to = (len / 2) + 1;
    println!("{:?}", to);
    &s[from..to]
}

fn main() {
    get_middle("test");
    get_middle("A");
    get_middle("testi");
}
// fn hor_mirror(s: String) -> String {
//     s.split('\n').rev().collect::<Vec<&str>>().join("\n")
// }
// fn vert_mirror(s: String) -> String {
//     s.split('\n')
//         .map(|s| s.chars().rev().collect::<String>())
//         .collect::<Vec<String>>()
//         .join("\n")
// }
// fn oper(oper: fn(String) -> String, s: String) -> String {
//     oper(s)
// }

#[test]
fn example_tests() {
    assert_eq!(get_middle("test"), "es");
    assert_eq!(get_middle("testing"), "t");
    assert_eq!(get_middle("middle"), "dd");
    assert_eq!(get_middle("A"), "A");
    assert_eq!(get_middle("of"), "of");
}
