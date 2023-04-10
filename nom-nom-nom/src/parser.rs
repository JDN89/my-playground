use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::error::VerboseErrorKind::Nom;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::delimited;

#[derive(Debug, PartialEq)]
pub struct Crate(char);

pub fn parse_crate(input: &str) -> IResult<&str, Crate> {
    //unwrap because the parser closure return an IResult and crate takes a char
    let first_char_closure = |s: &str| Crate(s.chars().next().unwrap());
    let parser = delimited(tag("["), take(1_usize), tag("]"));
// map -> aplly the second argument (function or closure) to the output of the parser (first arg)
    map(parser, first_char_closure)(input)
}

pub fn parse_hole(input: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(input)
}

// map crate parser to some and map hole parser to none

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    // delimited(parse_hole(i),parse_crate(i),parse_hole(i)) (i)
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

//we alternate between holes and crates -> there is an empty space - tag(" ") - on which we can alternate
fn parse_crates(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (input, result) = separated_list1(tag(" "), parse_crate_or_hole)(i)?;
    println!("{:?}",result);
    Ok((input, result))
}

// renamed to 👇 better indicate functionality
pub fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                // 👇
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub mod parser {
    use nom::combinator::all_consuming;
    use nom::Finish;
    use crate::parser::{parse_crates, transpose_rev};

    pub fn parse(input: &String) {
        let mut crate_lines = vec![];
        for line in input.lines() {
                if let Ok((_rest, crate_line))=all_consuming(parse_crates)(line).finish() {
                crate_lines.push(crate_line);
            };
        }
        let crate_columns = transpose_rev(crate_lines);
        for col in &crate_columns {
            println!("{col:?}");
        }

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
        assert_eq!(result, Ok(("", ())));
    }

    //TODO:fix test?
    #[test]
    fn test_parse_crate_or_hole_crate() {
        let input = "[D]";
        let result = parse_crate_or_hole(input);
        assert_eq!(result, Ok(("", Some(Crate('D')))));
    }

    #[test]
    fn test_parse_crate_or_hole_hole() {
        let input = "   ";
        let result = parse_crate_or_hole(input);
        assert_eq!(result, Ok(("", None)));
    }

    #[test]
    fn test_parse_crates() {
        let input = "[N] [C]    ";
        let result = parse_crates(input);
        assert_eq!(result,)
    }
    // parse crate alternate between crate and hole

    // #[test]
    // fn test_parse_crates() {
    //     let input = "[a][b][c]";
    //     let (_, result) = parse_crates(input).unwrap();
    //     assert_eq!(result, vec!["a", "b", "c"]);
    // }
}
