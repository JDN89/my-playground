// parse extern crate and hole 
// parse crate or hole
//        alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
// parse full line -> separated_list1

//parse number -> noms number parser
// parse stack number
// 

// delimited + tag = take 
// all_consuming
// filter_map to ignore the options
// preced tag, parse number isntruction

// When we're parsing crate lines, and we don't want to move out of the iterator, since we still have more stuff to parse after, instead of doing (&mut lines)
//Instead of popping each item from the end of the source stack, we can use drain with a range to drain only the part we need to move. Then, we can use extend on the destination stack to add everything from that iterator:


use std::mem::transmute;
use nom::combinator::all_consuming;
use nom::Finish;
use crate::parse::{parse_crate_line, parse_instruction};
use crate::helpers::transpose_rev;

mod parse;
mod helpers;




#[derive(Debug)]
struct Crate(char);

fn main() {
    let mut lines = include_str!("test.txt").lines();

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();
    let crate_columns = transpose_rev(crate_lines);
    for col in &crate_columns {
        println!("{col:?}");
    }

    // we've consumed the "numbers line" but not the separating line
    assert!(lines.next().unwrap().is_empty());

    let instructions: Vec<_> = lines
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
        .collect();
    for ins in &instructions {
        println!("{ins:?}");
    }
}



