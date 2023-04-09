use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::IResult;
use nom::sequence::delimited;

#[derive(Debug,PartialEq)]
pub struct Crate(char);

pub fn parse_crate(input: &str) -> IResult<&str, Crate> {
    //unwrap because the parser closure return an IResult and crate takes a char
    let first_char_closure = |s: &str| Crate(s.chars().next().unwrap());
    let parser = delimited(tag("["), take(1_usize), tag("]"));
// map -> aplly the second argument (function or closure) to the output of the parser (first arg)
    map(parser, first_char_closure)(input)
}

pub fn parse_hole(input: &str) -> IResult<&str,()> {
map(tag("   "),drop )(input)
}


// fn parse_crate_or_hole() {
//     delimited()
// }

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
    #[test]
    fn test_parse_hole() {
        let input = "   ";
        let result = parse_hole(input);
        assert_eq!(result,Ok(("",())));
    }

    // parse crate alternate between crate and hole

    // #[test]
    // fn test_parse_crates() {
    //     let input = "[a][b][c]";
    //     let (_, result) = parse_crates(input).unwrap();
    //     assert_eq!(result, vec!["a", "b", "c"]);
    // }
}
