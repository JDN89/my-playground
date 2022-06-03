// Rust maakt onderscheidt tussen recoverable en unrecoverable errors
// recoverable -> inform user something went wrong => Result <t,e>
// unrecoverable -> bug => panic!
// panic! -> print failure message, cunwind , clean up the stack and quit
fn main() {

let v = vec![1,2,3];
v[30];
}
