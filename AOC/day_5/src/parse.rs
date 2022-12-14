use std::fmt;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map, opt},
    sequence::{delimited, preceded},
    Finish, IResult,
};
use nom::bytes::complete::take_while1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::tuple;


/* [H] [M] [N] [Z] [M] [C] [M] [P] [P]
 1   2   3   4   5   6   7   8   9  */

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

#[derive( PartialEq, Eq)]
pub struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn parse_crate(input: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let parser = delimited(tag("["), take(1_usize), tag("]"));
    // map the result of the parser to the closure function
    map(parser, first_char)(input)
    // same as Ok((input,Crate))
}
// add code here



//parse hole
fn parse_hole(input: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(input)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}


// parse crate or hole
// alt function


// pub fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
//     separated_list1(tag(" "), parse_crate_or_hole)(i)
// }

pub fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_hole(i)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

// convert from 1-indexed to 0-indexed
fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

#[derive(Debug)]
pub struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

pub fn  parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
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

//push everything in piles
impl Piles {
    fn apply(&mut self, ins: Instruction) {
        for _ in 0..ins.quantity {
            let el = self.0[ins.src].pop().unwrap();
            self.0[ins.dst].push(el);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_crate() {
        let tests = [
            ("[M]   ", Crate('M')),
        ];
        for (input, expected_output) in tests {
            let    (_remaining_input, output) = parse_crate(input).unwrap();
            println!("{:?}", output);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_parse_crate_line() {
        let tests = [
            ("[W]     [G]", Crate('W')),
        ];
        for (input, expected_output) in tests {
            let    (_remaining_input, output) = parse_crate(input).unwrap();
            println!("{:?}", output);
            assert_eq!(output, expected_output);
        }
    }

}

// impl Line {
//
// }


// pub fn parse_input(s: &str) -> Vec<Line> {
//     let (remaining_input, lines) = separated_list1(line_ending, Line::parse)(s).unwrap();
//     assert!(remaining_input.is_empty());
//     lines
// }
