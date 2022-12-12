use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map, opt},
    sequence::{delimited, preceded},
    Finish, IResult,
};


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

#[derive(Debug, PartialEq, Eq)]
pub struct Crate(char);

fn parse_crate(input: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let parser = delimited(tag("["), take(1_usize), tag("]"));
    // map the result of the parser to the closure function
    map(parser, first_char)(input)
    // same as Ok((input,Crate))
}
// add code here

// fn line(input: &str) -> IResult<&str,>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_crate() {
        let tests = [
            ("[M]   ", Crate('M')),
        ];
        for (input, expected_output) in tests {
                     let    (remaining_input, output) = parse_crate(input).unwrap();
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
