fn main() {
    println!("Hello, world!");

    let mut x: u32 = 0;
    // can store result of looping expression as a statement
    let y = loop {
        x += 1;

        if x == 10 {
            // ending expression of loop
            break x;
        }
    };
    println!("y = {y}");

    // basic for loop
    let a = [1, 2, 3, 4];
    for elem in a {
        println!("{}", elem);
    }

    println!("---------");

    // range based for loop (works over half open range)
    for i in (1..4).rev() {
        println!("{i}");
    }
}
