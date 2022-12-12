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


mod parse;




#[derive(Debug)]
struct Crate(char);

fn main() {
    fn main() {
        // let lines = parse::parse_input(include_str!("../input.txt"));
        // for line in lines {
        //     println!("{:?}", line);
        // }
    }
 }

