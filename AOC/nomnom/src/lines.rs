use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::separated_pair;
use crate::parse::Pair;

//5-96,6-99


#[derive(Debug, Eq, PartialEq)]
pub struct Line(pub Pair, pub Pair);

impl Line {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let comma = tag(",");
        let parse_comma = separated_pair(Pair::parse, comma, Pair::parse);
        map(parse_comma, |(p1, p2)| Line(p1, p2))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let tests = [("5-96,6-99", Line(Pair { start: 5, end: 96 }, Pair { start: 6, end: 99 }), "",)
        ];
        for (input, expected_output, expected_remaining_input) in tests {
            let (remaining_input, output) = Line::parse(input).unwrap();
            assert_eq!(remaining_input, expected_remaining_input);
            assert_eq!(output, expected_output);
        }
    }
}

