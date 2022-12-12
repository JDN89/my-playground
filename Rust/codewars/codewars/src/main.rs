fn min_max(lst: &[i32]) -> (i32, i32) {
    (*lst.iter().min().unwrap(), *lst.iter().max().unwrap())
}

fn main() {
    min_max(&[1, 2, 3, 4, 5]);
}

#[cfg(test)]
mod tests {
    use super::min_max;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: &[i32], expected: (i32, i32)) {
        assert_eq!(min_max(arr), expected, "{ERR_MSG} with lst = {arr:?}")
    }

    #[test]
    fn fixed_tests() {
        for (arr, expected) in [
            (vec![1, 2, 3, 4, 5], (1, 5)),
            (vec![2334454, 5], (5, 2334454)),
        ] {
            dotest(&arr, expected)
        }
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
