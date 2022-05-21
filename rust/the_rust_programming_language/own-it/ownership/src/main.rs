// stack: LIFO
// data must have fixed size
// very organized
//HEAP:
// less organized => store data in free spot and get a pointer to that spot
// pointer has a fixed size and can be stored on the stack
// OWNERSHIP IS MANAGING HEAP DATA
// When your code calls a function, the values passed into the function (including,
//potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack.
//When the function is over, those values get popped off the stack.

//RULES
// - each value has a var that's called it's owner
// - each value can only have one owner at a time
// - when the owner goes out of scope the value gets dropped

//SCOPE
// tussen de { } zoals bij c# en JS

fn main() {
    // string literal on the heap
    let _s = "hello";
    // string type -> on the heap => size unknown
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let s1 = String::from("hello");
    // copy pointer of s1
    //s1 becomes invalid
    // s2 is valid => s1 was moved to s2
    // prinstln s1 doens't work because s1 is invalid after shallow copy
    let s2 = s1;

    println!("{}, world!", s1);
}

// string type
// - we need to allocate an ammoutn of memory on the heap for string type
// when value goes out of sscope, rust calls function drop that removes the value out of scope from the heap

//string iss made up of 3 parts
// pointer -> points to place in heap where it's store
// length
// capacity
// deze 3 waarden (pointer) worden in de stack gesaved
// de eigenlijke values of de string worden in de heap opgeslaan
// als je de String type kopieer keer je niet de eigenlijke waarden maar enkel de POINTER
// we DO NOT copy the data on the heap

// DATA types stored on the stack
//
//    All the integer types, such as u32.
//  The Boolean type, bool, with values true and false.
//All the floating point types, such as f64.
//The character type, char.
//Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
// je kan bovenstaande waarden kopieren en na de copy zijn ze nog steeds valid omdat ze op de stack gestored worde
// stack is effecient en kopieren van waarden vormt hier geen probleem

//declare String
// Pass String to a function as an argument -> first declarion of string goes out pf scope
// String goes out off scope after being passed to the function
