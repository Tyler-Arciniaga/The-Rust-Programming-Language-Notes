fn main() {
    let x = String::from("Tyler");
    immutable_borrow(&x);
    println!("In main: {x}");

    let mut y = String::new();
    mutable_borrow(&mut y);
    println!("{y}");

    let mut z = String::from("Arciniaga");

    // this is safe because we can have multiple immutable references
    let z1 = &z;
    let z2 = &z;
    println!("{z1} ... {z2}");

    // this is also safe, but z1 and z2 are now invalid, we can only have a single mutable
    // reference at any given time
    let z3 = &mut z;
    println!("{z3}");
}

// notice how some_string is never explicitely returned to caller (possible since
// function is borrowing some_string rather than having ownership passed to it)
fn immutable_borrow(some_string: &String) {
    println!("{some_string}")
}

fn mutable_borrow(some_string: &mut String) {
    some_string.push_str("Aloha");
}
