fn get_good_value_by_position(pos: usize) -> i32 {
    match pos {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 3,
        4 => 4,
        5 => 10,
        _ => 0,
    }
}

fn get_evil_value_by_position(pos: usize) -> i32 {
    match pos {
        0 => 1,
        1 => 2,
        2 => 2,
        3 => 2,
        4 => 3,
        5 => 5,
        6 => 10,
        _ => 0,
    }
}

fn get_forces_value(forces: &str, eval_position: fn(usize) -> i32) -> i32 {
    forces
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())
        .enumerate()
        .map(|(i, v)| eval_position(i) * v as i32)
        .sum()
}

fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_val = get_forces_value(good, get_good_value_by_position);
    let evil_val = get_forces_value(evil, get_evil_value_by_position);

    let result = good_val - evil_val;
    if result < 0 {
        String::from("Battle Result: Evil eradicates all trace of Good")
    } else if result > 0 {
        String::from("Battle Result: Good triumphs over Evil")
    } else {
        String::from("Battle Result: No victor on this battle field")
    }
}

fn main() {
    good_vs_evil("1 0 0 0 0 0", "0 0 0 0 1 0 0");
}

#[test]
fn returns_expected() {
    assert_eq!(
        good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"),
        "Battle Result: Good triumphs over Evil"
    );
    assert_eq!(
        good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"),
        "Battle Result: Evil eradicates all trace of Good"
    );
    assert_eq!(
        good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"),
        "Battle Result: No victor on this battle field"
    );
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
