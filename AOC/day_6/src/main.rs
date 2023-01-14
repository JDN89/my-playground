use crate::position_marker::find_marker;

mod position_marker;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();

     find_marker(&file);
}

