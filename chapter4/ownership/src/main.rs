fn main() {
    let x = "string literal"; // string literal pushed to stack
    println!("{x}");

    let mut y = String::from(""); // string type allocated on heap
    y.push_str("im on the heap!");
    println!("{y}");

    let my_string = String::from("tyler");
    take_ownership(my_string); // my_string is moved here

    // invalid line since ownership has been moved already
    // println!("{my_string}");

    let mut my_string_2 = String::from("random");
    my_string_2 = take_and_give_back(my_string_2);
    println!("{my_string_2}");
}

fn take_ownership(my_string: String) {
    println!("{my_string}");
} // my_string leaves scope, thus drop is called

fn take_and_give_back(my_string: String) -> String {
    // takes ownership but through returning my_string returns ownership to caller
    my_string
}

fn copying_and_single_ownership() {
    // the following is invalid since x is moved to y which calls drop on string when exiting its
    // scope and then x tries to use the moved value (should use clone method instead)
    //
    // let x = String::from("");
    // {
    //     let y = x;
    // }
    //
    // println!("{x}");
}
