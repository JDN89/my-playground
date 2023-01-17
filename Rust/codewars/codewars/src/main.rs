fn reverse_letters(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .rev()
        .collect::<String>()
}

fn main() {
    reverse_letters("man1");
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
#[cfg(test)]
mod tests {
    use super::reverse_letters;

    fn dotest(s: &str, expected: &str) {
        let actual = reverse_letters(s);
        assert!(
            actual == expected,
            "With s = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("krishan", "nahsirk");
        dotest("ultr53o?n", "nortlu");
        dotest("ab23c", "cba");
        dotest("krish21an", "nahsirk");
    }
}
