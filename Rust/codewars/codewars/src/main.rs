fn high(input: &str) -> &str {
    let v: Vec<&str> = input.split(" ").collect();
    let mut vec_weight_word = Vec::new();

    for w in &v {
        let ascii_values: i32 = w.chars().map(|c| c as i32 - 96).sum();
        vec_weight_word.push(ascii_values);
        println!("word: {:?}", ascii_values);
    }

    let max_value = vec_weight_word.iter().max().unwrap();

    let position_max_word = vec_weight_word.iter().position(|i| i == max_value).unwrap();
    println!("{:?}", position_max_word);
    v[position_max_word]
}

fn main() {
    high("man i need a taxi up to ubud");
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
    use super::high;

    #[test]
    fn test_basic() {
        assert_eq!(high("man i need a taxi up to ubud"), "taxi");
        assert_eq!(high("what time are we climbing up the volcano"), "volcano");
        assert_eq!(high("take me to semynak"), "semynak");
        assert_eq!(high("massage yes massage yes massage"), "massage");
        assert_eq!(high("take two bintang and a dance please"), "bintang");
        assert_eq!(high("aa b"), "aa");
        assert_eq!(high("b aa"), "b");
        assert_eq!(high("bb d"), "bb");
        assert_eq!(high("d bb"), "d");
        assert_eq!(high("aaa b"), "aaa");
    }
}
