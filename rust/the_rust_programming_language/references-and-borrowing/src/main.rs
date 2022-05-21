// welk probleem lost referencing op
// Zonder reference & zou s1 uit scope vallen eens de fn calculate_length functie afgerond is
// reference
fn main() {
    let s1 = String::from("Hello");

    // zonder reference zou s1 uit scope vallen nadat het doorgegeven wordt als een argument in onderstaande functie
    let len = calculate_length(&s1);
    // zonder reference zou s1 niet in de println!("") macro kunnen gebruikt worden;
    println!("the length of {} is {}", s1, len);

    // ================= MUTABLE REFERENCE=======
    //ref is standaard immutable => make it mutable , add mut keyword
    // Mutable references have one big restriction: you can have only one MUTABLE reference to a particular piece of data at a time.
}

fn calculate_length(s: &String) -> usize {
    //no ; at the end because you return the legnth
    s.len()
    // here gaat s uit scope, maar omdat  het om een reference gaat wordt s niet invalid in the main functie
    // de ownerhsip werd niet overgenomen door de calculate_length functie
    // dus we moete de val niet returnen omdat we de ownerhsip nooit overmanem => borrowing
}
