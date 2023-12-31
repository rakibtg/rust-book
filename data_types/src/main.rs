fn main() {
    // Rust mainly has two different types of types
    // 1. scalar types
    // 2. compound types

    // Scalar types
    // A scalar type represents a single value.
    // Rust has four primary scalar types:
    //  1. integers
    let a: u8 = 1; // unsigned
    let b: i8 = -1; // signed
    let c = 200; // default -32, signed
    println!("Unsigned: {a}, Signed: {b}, Default signed: {c}");

    //  2. floating-point numbers
    let d = 1.2; // default f64 on moodern cpu
    let e: f64 = 2.4; // 64 bit floating number
    let f: f32 = 4.6; // 32 bit floating number
    println!("Default f64: {d}, 64 bit: {e}, 32 bit: {f}");

    //  3. booleans
    let g = true;
    let h: bool = false; // with explicit type annotation.
    println!("Booleans: {g}, {h}");

    //  4. characters
    let i = 'C';
    let j: char = '😻';
    println!("Chars: {i}, {j}");


    // Compound types: group of multiple values into one type.
    // 1. Tuple type: can have multiple types of value.
    let tup : (i32, i32, f32) = (2, 42, 9.53);
    let (t1, t2, t3) = tup;
    println!("Tuple values: {t1}, {t2} and {t3}");

    // 2. Array: Can only have same type accross all the values/item.
    let arr = [1, 24, 52];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    let first = arr[0];
    let first_month: &str = months[0];
    println!("First array: {first}, first month: {first_month}");

    // define array size.
    let arr_with_size = [3; 5];
    // same as: let a = [3, 3, 3, 3, 3];
    // or
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // let month_index = 20;
    // let invalid_month = months[20];
    // println!("Invalid month: {invalid_month}");

    let t_tup = ([1; 2], [3; 4]);
    /*
        t_tup is initialized as a tuple containing two arrays.
        [1; 2] is an array with two elements, both set to 1. It's using Rust's array initialization syntax where the first number is the value and the second is the count.
        [3; 4] is an array with four elements, all set to 3.
        So, t_tup is a tuple where the first element is [1, 1] and the second is [3, 3, 3, 3].
    */
    let (a_tup, b_tup) = t_tup;
    println!("{}", a_tup[0]);
    println!("{}", a_tup[1]);
    println!("{}", b_tup[0]);

}
