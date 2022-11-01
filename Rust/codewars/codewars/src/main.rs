fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
    let expected: i32;

    match operator {
        '+' => expected = value1 + value2,
        '-' => expected = value1 - value2,
        '/' => expected = value1 / value2,
        '*' => expected = value1 * value2,
        _ => unreachable!(),
    }
    return expected;
}

fn main() {
    let result = basic_op('-', 4, 5);
    println!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::basic_op;

    fn dotest(op: char, v1: i32, v2: i32, expected: i32) {
        let actual = basic_op(op, v1, v2);
        assert!(actual == expected,
            "With operator = '{op}', value1 = {v1}, value2 = {v2}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn example_tests() {
        dotest('+', 4, 7, 11);
        dotest('-', 5, 4, 1)
    }
}
