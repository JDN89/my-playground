/* Rust range .contains

1 2
1 2 3
for on firsts group
1 2 3
1 2
how to check this?
Check wich range is the biggest?
- count range
- smallest is the group we loop over
- check if biggest group contains i
- if contains -> result +=1 */
/* unpack range
store in array
put into array */
use std::fs;

#[derive(Debug)]
struct Pair {
    start: i32,
    end: i32,
}
impl Pair {
    fn total_range(&self) -> i32 {
        let mut range = 0;
        for _num in self.start..self.end + 1 {
            range += 1
        }
        range
    }
}

fn check_in_range(x: Pair, y: Pair) -> Vec<i8> {
    // println!("trigger");
    let range_fp = x.total_range();
    let range_sp = y.total_range();

    let mut in_range: Vec<i8> = Vec::new();

        if range_fp == range_sp {
            if x.start == y.start && x.end == y.end {
                in_range.push(1)
            }
            in_range.push(0)
        } else if range_fp > range_sp {
            for num in y.start..y.end+1 {
                if (x.start..x.end).contains(&num) {
                    in_range.push(1);
                }
                in_range.push(0);
            }
        } 

        // else if range_fp < range_sp {
            for num in x.start..x.end {
                if (y.start..y.end).contains(&num) {
                    in_range.push(1);
                }
                in_range.push(0);
            }
        // }
             println!("{:?}",in_range);
        in_range

}

fn cleanup(all_pairs: String) -> i32 {
    let mut result = 0;
    for pair in all_pairs.lines() {
        let pair: (&str, &str) = pair.split_once(",").unwrap();
        let fp: (&str, &str) = pair.0.split_once("-").unwrap();
        let sp: (&str, &str) = pair.1.split_once("-").unwrap();

        let frst_p = Pair {
            start: fp.0.parse::<i32>().unwrap(),
            end: fp.1.parse::<i32>().unwrap(),
        };
        let scnd_p = Pair {
            start: sp.0.parse::<i32>().unwrap(),
            end: sp.1.parse::<i32>().unwrap(),
        };

let in_range = check_in_range(frst_p, scnd_p);
result +=1



    }
    result
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    cleanup(file);
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(
            cleanup("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string()),
            2
        );
        // assert_eq!(rock_paper_siccors_part1("A Y\nB X\nC Z".to_string()), 15);
    }
}
