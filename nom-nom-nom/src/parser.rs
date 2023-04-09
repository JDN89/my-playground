use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::IResult;
use nom::sequence::delimited;

#[derive(Debug,PartialEq)]
pub struct Crate(char);

pub fn parse_crate(input: &str) -> IResult<&str, Crate> {
    let first_char_closure = |s: &str| Crate(s.chars().next().unwrap());
    let crate_parser_function = delimited(tag("["), take(1_usize), tag("]"));
    map(crate_parser_function, first_char_closure)(input)
}


pub mod parser {

    pub fn parse(input: &String) {
        // for line in input.lines() {
        //     let parsed_line = parse_crates(line).expect("parse crate lines");
        //     println!("{:?}", parsed_line);
        // }
        println!("{input}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_crate() {
        let input = "[A]";
        let result = parse_crate(input);
        assert_eq!(result, Ok(("", Crate('A'))));
    }

    // #[test]
    // fn test_parse_crates() {
    //     let input = "[a][b][c]";
    //     let (_, result) = parse_crates(input).unwrap();
    //     assert_eq!(result, vec!["a", "b", "c"]);
    // }
}
