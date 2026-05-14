fn main() {
    let mut x = 5; //immutable by defualt
    println!("Value: {x}");
    x = 6;
    println!("Value: {x}");

    const SOME_CONST_VALUE: u32 = 7 * 7 * 7;
    println!("Const Value: {SOME_CONST_VALUE}");

    let y = 10;
    let y = y + 1;
    {
        let y = y * 2;
        println!("Shadowed Y value: {y}");
    }
    println!("Current Y value: {y}");

    let tup = ("Tyler", 777, true);
    let (_, number, _) = tup;
    println!("{} = {number}", tup.1);

    let arr = [1, 2, 3, 4, 5, 6, 7];
    let arr2 = [7; 3]; // [7, 7, 7]

    println!("{}", arr[3]);
    println!("{}", arr2[1]);
}
