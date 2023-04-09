// look at nom and which libaries to use to do the challange
// parse the crates CHAR
// put them in vectors
// flip the vectors
// parse the commands with TAG
mod parser; 
mod nom_test;

fn main() {

   let input =  read_input().unwrap();
   println!("{input}");
   
   parser::parser::parse(&input);
}

fn read_input() -> Result<String,std::io::Error> {
Ok(std::fs::read_to_string("src/test-input.txt")?)

}
