// al de values in vector zijn van zelfde type
// je kan naar value in vector verwijzen via . en get
// get geeft non value als je naar index buiten vector range verwijst
// . panicked
// je kan geen mutable and immutable ref hebben in dezelfde scope
// waarom: als je vector muteert en extra values toevoegt
// kan het zijn dat je de values moet kopieren naar een nieuze plaats in memory
// omdat al de values naast elkaar moeten staan en op de oude lcoatie niet genoeg ruimte was
// dus je oude pointer zou in dat geval niet meer geldig zijn! ->
fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3, 4, 5];
}
