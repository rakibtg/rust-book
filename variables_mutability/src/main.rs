fn main() {
    // immutable variables.
    let y = 10;
    println!("Value of y: {y}");

    // mutable variables.
    // - use the `mut` keyword to be able to change the value later.
    let mut x = 5;
    println!("Value of x: {x}");
    x = 6;
    println!("Value of x: {x}");

    // constants
    // - const can be declared in any scope, including global scope so other scopes can share common data.
    // - no value that gets computed at runtime.
    // - the constant variable should always be typed because rust can not infer the type of the value of a constant.
    // - a constant can never be mutable.
    // - constant naming convention is all uppercase with underscore in between words.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    // - defining same vairable multiple times.
    // - compiler only see the last defined variable.
    let a = 5;
    let a = a + 1;
    {
        let a = a * 2;
        println!("Value of a within inner-scope: {a}");
    }
    println!("Value of a: {a}");

    // - with shadowing you can change the type of the variable.
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Total spaces: {spaces}");

    // - with mutable you can not change the type of the variable.
    /*
        let mut secondSpaces = "    ";
        secondSpaces = secondSpaces.len();
    */
}
