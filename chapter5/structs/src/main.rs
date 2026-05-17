struct Person {
    name: String,
    age: u32,
    lives_in_nyc: bool,
}

// struct tuple
struct Point(i32, i32, i32);

fn main() {
    let age: u32 = 21;

    // can shorthand a struct field value if variable has same name as field (see age field)
    let t = Person {
        name: String::from("Tyler"),
        age,
        lives_in_nyc: true,
    };

    // when copying structs you can shorthand with .._t to say all other fields are copied/moved
    let _copy = Person { age: 45, ..t };

    // this is invalid because t has a field with String and thus copying such field into _copy
    // caused it to be moved, thus t does not own such a string and is dropped
    // println!("{} {} {}", t.name, t.age, t.lives_in_nyc);

    let some_point = Point(0, -1, 43);
    let Point(x, y, z) = some_point;
}
