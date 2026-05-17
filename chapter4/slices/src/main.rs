fn main() {
    let x = "My string. I like it a lot!";
    let y = first_word(x);
    println!("{y}");
}

fn first_word(some_string: &str) -> &str {
    let bytes = some_string.as_bytes();
    for (i, &elem) in bytes.iter().enumerate() {
        if elem == b' ' {
            return &some_string[..i];
        }
    }

    some_string
}
