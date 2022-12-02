// use std::fs;
//
// fn main() {
//   let contents = fs::read_to_string("input.txt")
//         .expect("Should have been able to read the file");
//
//       println!("With text:\n{contents}");
//
//       let reader = BufReader
//  }

use std::fs::{self};
// use std::io::{self, prelude::*, BufReader};

fn main() {
    // let file = File::open("input.txt")?;
    // let reader = BufReader::new(file);
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut sum_array: Vec<u32> = Vec::new();
    let mut calories = 0;

    for line in contents.lines() {
        let my_string = &line.to_string();

        // if my_string.is_empty() == true {
        //     sum_array.push(calories)
        // } else {
        //     calories += my_string.parse::<u32>().unwrap();
        // }

                match my_string.len() {

                    0 => {sum_array.push(calories) ;
                        calories = 0 },
        
                    _ => calories= calories + my_string.parse::<u32>().unwrap()

                }
    }

    sum_array.sort();
    sum_array.reverse();
    for calory in &sum_array {
        println!("{}", calory)
    }
    let max_calories = sum_array.iter().max().unwrap();
    println!("Most calories carried by an elf: {:?}", max_calories);
    let final_array = sum_array; 


     let top_three_elves = &final_array[0] + final_array[1] + final_array[2];

     println!("Calories carried by the top three elves: {}",top_three_elves);

}
