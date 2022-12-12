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


use core::fmt;
use std::fs;
use nom::{combinator::map_res, bytes::complete::take_while_m_n, sequence::tuple};
pub use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alpha1, digit1, multispace1, newline, space1,
    },
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    *,
};


#[derive(Debug)]
struct Crate(char);

fn main() {
 }


#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

struct Piles(Vec<Vec<Crate>>);

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}


/* #[cfg(test)]
mod tests {

    use super::*;
     const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
\nmove 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn returns_expected() {
        assert_eq!(
hex_color(INPUT.to_string()),"CMZ")
    }
} */
