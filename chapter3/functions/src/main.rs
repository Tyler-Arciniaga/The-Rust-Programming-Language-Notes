// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.
//
//
// Functions can (and normally do) end with an ending expression (can also return early with return
// keyword)

fn main() {
    println!("Hello, world!");
    second_function(5, "tyler");

    let y = {
        let x = 7; // statement: ends with semicolon
        x + 5 // expression: evals to a value, does not end with semicolon
    };

    println!("y = {y}");

    let z = third_function(10);
    println!("z = {z}");
}

fn second_function(val: u32, name: &str) {
    println!("{val} : {name}");
}

fn third_function(val: u32) -> u32 {
    val + 1
}
