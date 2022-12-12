use nom::{
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use crate::lines::Line;

pub fn parse_numbers(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str) (input)
    
}

#[derive(Debug,PartialEq,Eq)]
pub struct Pair {
   pub start: u32,
    pub end: u32,
}

impl Pair {
    pub fn parse(input: &str) -> IResult<&str,Self> {
        let parse_two_numbers = separated_pair(parse_numbers, char('-'),parse_numbers);
        map(parse_two_numbers, |(start,end)| Pair{start,end}) (input)
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pair() {
        let tests = [
            ("5-96", Pair { start: 5, end: 96 }, ""),
            ("6-99asdf", Pair { start: 6, end: 99 }, "asdf"),
        ];
        for (input, expected_output, expected_remaining_input) in tests {
            let (remaining_input, output) = Pair::parse(input).unwrap();
            assert_eq!(output, expected_output);
            assert_eq!(remaining_input, expected_remaining_input);
        }
    }
}

pub fn parse_input(s: &str) -> Vec<Line> {
    let (remaining_input, lines) = separated_list1(line_ending, Line::parse)(s).unwrap();
     assert!(remaining_input.is_empty());
    lines
}


//5-96,6-99
