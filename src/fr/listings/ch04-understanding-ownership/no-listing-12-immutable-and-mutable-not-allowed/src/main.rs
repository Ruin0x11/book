fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // sans problème
    let r2 = &s; // sans problème
    let r3 = &mut s; // GROS PROBLEME
    
    println!("{}, {}, and {}", r1, r2, r3);
    // ANCHOR_END: here
}
