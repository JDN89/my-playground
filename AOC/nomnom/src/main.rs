use nom::combinator::all_consuming;
use crate::lines::Line;

mod parse;
mod lines;
//5-96,6-99

/* fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) =
        separated_list1(tag(" "), parse_line)(input)?;

    println!(Ok(result));
    Ok((input, result))
} */

fn main() {
    let lines = parse::parse_input(include_str!("../input.txt"));
        for line in lines {
            println!("{:?}", line);
        }
}
/* #[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(
            cleanup_part_two("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string()),
            4
        );
    }
} */
