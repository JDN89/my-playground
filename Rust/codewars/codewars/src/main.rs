fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

fn main() {
    array_diff(vec![1, 2, 2], vec![1]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
        assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
        assert_eq!(array_diff(vec![1, 2, 3], vec![1, 2]), vec![3]);
    }
}

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
//f
//
//     return "Hello".to_string()

// }
/* use itertools::Itertools;

fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut numbers = numbers.to_vec();
    match numbers.iter().position_min() {
        None => numbers,
        Some(m) => {
            numbers.remove(m);
            numbers
      }
    }
} */

// fn largest_five_digit_number(num: &str) -> u32 {
//     let mut max: u32 = 0;
//     let len_minus_4 = num.len() - 4;
//     let total_len = num.len();
//
//     println!("{}", len_minus_4);
//     println!("{}", total_len);
//
//     for i in 0..num.len() - 4 {
//         println!("{}",i);
//         let value = num.get(i..i + 5).unwrap().parse().unwrap();
//         if value > max {
//             max = value
//         };
//     }
//     max
// }
//
// ===========================
// iterate over Vec and return only non matching values

// fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
//
//     a.retain(|i| !b.contains(i)).collect();
// }

// fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
//     a.into_iter().filter(|x| !b.contains(x)).collect()
// }
