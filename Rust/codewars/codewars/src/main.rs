
// fn alphabet_position(text: &str) -> String {
//     
//     //https://en.wikipedia.org/wiki/ASCII#Printable_characters
//     text
//         .to_lowercase()
//         .chars()
//         .filter( |c|  c >= &'a' && c <= &'z')
//         .map(|c| (c as u32 -96).to_string())
//         .collect::<Vec<String>>()
//         .join(" ")
//     // Code here...
//     
// }


// fn spin_words(words: &str) -> String {

// let result = words.split_ascii_whitespace().map(|word| match word.len() > 4 {
//         true => word.chars().rev().collect(),
//         false => word.to_string()

//      }  ).collect::<Vec<String>>().join(" ");

//     println!("{:?}",result);
//     
//         
//     return "Hello".to_string()


// }
use itertools::Itertools;

fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut numbers = numbers.to_vec();
    match numbers.iter().position_min() {
        None => numbers,
        Some(m) => {numbers.remove(m); numbers}
    }
}




fn main() {
     remove_smallest(&[1, 2, 3, 4, 5]);
}

#[cfg(test)]
mod tests {
    use super::*;
        const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(a: &[u32], expected: &[u32]) {
        assert_eq!(remove_smallest(a), expected, "{ERR_MSG} with numbers = {a:?}")
    }
    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5], &[2, 3, 4, 5]);
        dotest(&[1, 2, 3, 4], &[2, 3, 4]);
        dotest(&[5, 3, 2, 1, 4], &[5, 3, 2, 4]);
        dotest(&[1, 2, 3, 1, 1], &[2, 3, 1, 1]);
    }

}
