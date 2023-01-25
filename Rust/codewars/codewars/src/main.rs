fn sum_cubes(n: u32) -> u32 {
    //(1..=n) is an iterator from 1 until n
    (1..=n).map(|n| n.pow(3)).sum()
}

fn main() {
    sum_cubes(3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sum_cubes(1), 1);
        assert_eq!(sum_cubes(2), 9);
        assert_eq!(sum_cubes(3), 36);
        assert_eq!(sum_cubes(4), 100);
        assert_eq!(sum_cubes(10), 3_025);
        assert_eq!(sum_cubes(123), 58_155_876);
    }
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

// fn get_good_value_by_position(pos: usize) -> i32 {
//     match pos {
//         0 => 1,
//         1 => 2,
//         2 => 3,
//         3 => 3,
//         4 => 4,
//         5 => 10,
//         _ => 0,
//     }
// }

// fn get_evil_value_by_position(pos: usize) -> i32 {
//     match pos {
//         0 => 1,
//         1 => 2,
//         2 => 2,
//         3 => 2,
//         4 => 3,
//         5 => 5,
//         6 => 10,
//         _ => 0,
//     }
// }

// fn get_forces_value(forces: &str, eval_position: fn(usize) -> i32) -> i32 {
//     forces
//         .split(" ")
//         .map(|s| s.parse::<i32>().unwrap())
//         .enumerate()
//         .map(|(i, v)| eval_position(i) * v as i32)
//         .sum()
// }

// fn good_vs_evil(good: &str, evil: &str) -> String {
//     let good_val = get_forces_value(good, get_good_value_by_position);
//     let evil_val = get_forces_value(evil, get_evil_value_by_position);

//     let result = good_val - evil_val;
//     if result < 0 {
//         String::from("Battle Result: Evil eradicates all trace of Good")
//     } else if result > 0 {
//         String::from("Battle Result: Good triumphs over Evil")
//     } else {
//         String::from("Battle Result: No victor on this battle field")
//     }
// }
