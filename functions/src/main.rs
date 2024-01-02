fn main() {
    println!("Hello, world!");
    another_function();
    get_value(100, "thousand");
    let incremented_number = get_incremented_number(9);
    println!("Incremented number: {incremented_number}");
}

// Defining a simple function.
fn another_function() {
    println!("From Another Function!");
}

// Parameters with type. In this example it is unsigned 32bit.
// We must declare the type of the parameter
fn get_value(val: u32, unit_label: &str) {
    println!("The value is: {val} {unit_label}");
}

/*
    IMPORTANT: There are two important concept in rust for functions,
        1. Statemnet: Statement do not return values.
        2. Expression:
            -   Calling a function is an expression.
            -   A scope created with curly braces is an expression.
            -   Expression does not end with a semicolon,
                adding the semicolon turns it into a statement. So it will not return the value.
*/

// Annotating return type in rust funciton
fn get_incremented_number(num: i32) -> i32 {
    num + 1
}
