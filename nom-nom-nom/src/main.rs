mod parser; 

fn main() {

   let input =  read_input().unwrap();
   println!("{input}");
   
   parser::parser::parse()
}

fn read_input() -> Result<String,std::io::Error> {
Ok(std::fs::read_to_string("src/test-input.txt")?)

}
