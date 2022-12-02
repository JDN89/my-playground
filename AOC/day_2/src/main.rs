/*
A - Y
B - X
C - Z */

/* rock 1 - Y
paper 2 - X
scissors 3 - Z */

// lose = 0
// draw = 0
// win = 6

use std::{collections::HashMap, fs};

fn rock_paper_siccors_part1(file: String) -> i32 {
    let rock = 1;
    let paper = 2;
    let scissors = 3;

    let win = 6;
    let draw = 3;
    let lose = 0;

    let mut rock_paper_siccors = HashMap::new();
    rock_paper_siccors.insert('A', rock);
    rock_paper_siccors.insert('B', paper);
    rock_paper_siccors.insert('C', scissors);
    rock_paper_siccors.insert('X', rock);
    rock_paper_siccors.insert('Y', paper);
    rock_paper_siccors.insert('Z', scissors);

    let mut score = 0;
    let mut counter = 0;

    for line in file.lines() {
        counter += 1;
        if line.is_empty() {
            break;
        }
        let result = line.split_once(" ").unwrap();
        let op = result.0.parse::<char>().unwrap();
        let me = result.1.parse::<char>().unwrap();

        let op_value_choice = rock_paper_siccors.get(&op).unwrap();
        let me_value_choice = rock_paper_siccors.get(&me).unwrap();

        if op_value_choice == me_value_choice {
            score = score + draw + me_value_choice
        } else if op_value_choice.to_owned() == 1 && me_value_choice.to_owned() == 2
            || op_value_choice.to_owned() == 2 && me_value_choice.to_owned() == 3
            || op_value_choice.to_owned() == 3 && me_value_choice.to_owned() == 1
        {
            score = score + win + me_value_choice
        } else {
            score = score + me_value_choice
        }

        // println!("{:?} : {:?} ", op,me);
        println!("{:?} : {} ", score, counter);
    }

    println!("{}", score);
    return score;
}

fn rock_paper_siccors_part2(file: String) -> i32 {
    let op_point: [i32; 3] = [1, 2, 3];

    let rock = 1;
    let paper = 2;
    let scissors = 3;

    let win = 6;
    let draw = 3;
    let lose = 0;

    let mut winning = HashMap::new();
    winning.insert('A', paper);
    winning.insert('B', scissors);
    winning.insert('C', rock);

    let mut losing = HashMap::new();
    losing.insert('A', scissors);
    losing.insert('B', rock);
    losing.insert('C', paper);

    /* let mut drawing = HashMap::new();
    losing.insert('A', 1);
    losing.insert('B', 2);
    losing.insert('C', 3); */

    let drawing = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);

    let mut rock_paper_siccors = HashMap::new();
    rock_paper_siccors.insert('X', lose);
    rock_paper_siccors.insert('Z', win);
    rock_paper_siccors.insert('Y', draw);

    let mut score = 0;
    let mut counter = 0;

    for line in file.lines() {
        counter += 1;
        if line.is_empty() {
            break;
        }
        let result = line.split_once(" ").unwrap();
        let op = result.0.parse::<char>().unwrap();
        let me = result.1.parse::<char>().unwrap();

        let me_value_choice = rock_paper_siccors.get(&me).unwrap();

        if me_value_choice.to_owned() == draw {
            score = score + draw + drawing.get(&op).unwrap()
        } else if me_value_choice.to_owned() == win {
            score = score + win + winning.get(&op).unwrap()
        } else {
            score = score + lose + losing.get(&op).unwrap()
        }

        println!("{}", score);
    }

    println!("{}", score);
    return score;
}
fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    rock_paper_siccors_part2(file);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        // assert_eq!(rock_paper_siccors("A X".to_string()), 8);
        // assert_eq!(rock_paper_siccors_part1("A Y\nB X\nC Z".to_string()), 15);
        assert_eq!(rock_paper_siccors_part2("A Y\nB X\nC Z".to_string()), 12);
    }
}
