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
#[derive(Debug,PartialEq)]
pub struct Color {
  pub red:     u8,
  pub green:   u8,
  pub blue:    u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
  map_res(
    take_while_m_n(2, 2, is_hex_digit),
    from_hex
  )(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
  let (input, _) = tag("#")(input)?;
  let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

  Ok((input, Color { red, green, blue }))
}


fn main() {
 match hex_color("#2F14DF") {
 Ok(sk) => { 
     println!("{:?}",sk.1)   // ... use sk ...
    },
    Err(e) => {
        println!("{:?}",e)
        // ... sk is not available, and e explains why ...
    },
 }


  assert_eq!(hex_color("#2F14DF"), Ok(("", Color {
    red: 47,
    green: 20,
    blue: 223,
  })));
    /* let file = fs::read_to_string("input.txt").unwrap();
    // cleanup(file);
     stack_organizer(file); */
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
