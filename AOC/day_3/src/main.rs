// read lines and split each line into two parts
// https://en.wikipedia.org/wiki/ASCII#Printable_characters
// A-Z -> range in ASCII : 65 to 90
// a-z -> range in ASCII : 97 to 122
// convert to position in alphabet for capital letters it's x2
// add priority to result (total sum)

use std::fs;

/* fn find_duplicates_set(mut nums: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32,i32> = HashMap::new();
    let mut doubles: HashSet<i32> = HashSet::new();
    println!("{:?}",nums);
    if nums.len()== 0 {
        return Vec::new();
    }
    for i in nums {
        let v = map.entry(i).or_insert(0);
        *v +=1;
        if *v > 1 {
            doubles.insert(i);
        }
    }
    doubles.into_iter().collect()
} */

fn rukzak_organizer(rukzakken: String) -> i32 {
    let mut sum_priorities = 0;
    for rukzak in rukzakken.lines() {
        let ruk_vec: Vec<char> = rukzak.chars().collect();

        let rukzak_middle = rukzak.len() / 2;
        let (comp_one, comp_two) = ruk_vec.split_at(rukzak_middle);
        let common_item: Vec<&char> = comp_one
            .into_iter()
            .filter(|c| comp_two.contains(c))
            .collect();

        println!("{:?}", common_item[0]);

        if common_item[0] >= &'a' && common_item[0] <= &'z' {
            let lower_priority = common_item[0].to_owned() as i32 - 96;

            sum_priorities += lower_priority;

            println!("lower priority: {:?}", lower_priority);
        }

        if common_item[0] >= &'A' && common_item[0] <= &'Z' {
            let upper_priority = common_item[0].to_owned() as i32 - 64;

            sum_priorities += upper_priority + 26;

            println!("upper priority: {:?}", upper_priority + 26);
        }
        /* let lower_positions = rukzak
            .chars()
            .filter(|c| c >= &'a' && c <= &'z')
            .map(|c| (c as i32 - 96))
            .collect::<Vec<i32>>();

        let upper_positions = rukzak
            .chars()
            .filter(|c| c >= &'A' && c <= &'Z')
            .map(|c| (c as i32 - 64))
            .collect::<Vec<i32>>(); */
    }

    println!("sum_priorities : {:?}", sum_priorities);
    sum_priorities
}

// PART II

fn group_organizer(rukzakken: String) -> i32 {
    let mut sum_priorities = 0;
    let mut group: Vec<String> = Vec::new();
    let mut groups: Vec<Vec<String>> = Vec::new();

    let mut counter = 1;
    for rukzak in rukzakken.lines() {
        group.push(rukzak.to_owned());
        if counter % 3 == 0 {
            let new_group = &group;
            groups.push(new_group.to_owned());
            group.clear();
            counter += 1;
        } else {
            counter += 1;
        }
    }

    for group in groups  {

        let one: Vec<char> = group[0].chars().collect();
        let two: Vec<char> = group[1].chars().collect();
        let three: Vec<char> = group[2].chars().collect();

        let common_item : Vec<char> = 
            one
            .into_iter()
            .filter(|c| two.contains(c) && three.contains(c))
            .collect();
    println!("common : {:?}", common_item);

        if common_item[0] >= 'a' && common_item[0] <= 'z' {
            let lower_priority = common_item[0].to_owned() as i32 - 96;

            sum_priorities += lower_priority;

            println!("lower priority: {:?}", lower_priority);
        }

        if common_item[0] >= 'A' && common_item[0] <= 'Z' {
            let upper_priority = common_item[0].to_owned() as i32 - 64;

            sum_priorities += upper_priority + 26;

            println!("upper priority: {:?}", upper_priority + 26);
        }
        
    }

    println!("common : {:?}", sum_priorities);
    sum_priorities
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    group_organizer(file);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        // assert_eq!(rock_paper_siccors("A X".to_string()), 8);
        // assert_eq!(rock_paper_siccors_part1("A Y\nB X\nC Z".to_string()), 15);
        // assert_eq!(rock_paper_siccors_part2("A Y\nB X\nC Z".to_string()), 12);
        // assert_eq!(
        //     rukzak_organizer(
        //         "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg"
        //             .to_string()
        //     ),
        //     96
        // )
        assert_eq!(
            group_organizer(
                "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"
                    .to_string()
            ),
            70
        )
    }
}
