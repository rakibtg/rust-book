fn main() {
    let num = 3;

    if num < 5 {
        println!("Condition is true");
    } else if num != 0 {
        println!("Condition is not valid");
    } else {
        println!("Condition is false");
    }

    /*

        // IMPORTANT: Conditions must be booleans. For example below code will get an error:

        let val = 5;
        if val { // "val" is not boolean type.
            println!("Number exists!");
        }

     */

     // Using if in a let statement like a turnary operator alternative.
     let condition = true;
     let number = if condition { 5 } else { 6 }; // The value from both check need to be same type.
     println!("Number is {number}");

     let x; // Here we are just declaring the variable but not binding it hence immutability will not effect it!!
     if condition {
       x = 1;
     } else {
       x = 2;
     }
     println!("Value of x: {x}");

     // LOOPS
     // Two important things:
     //     - break: breaks out from the loop.
     //     - continue: skips the code below it and starts next iteration.

     loop {
         println!("Again!");
         break;
     }

     // Loop can return a value!
     let mut counter = 0;
     let result = loop {
         counter += 1;
         if counter == 10 {
             break counter * 2;
         }
     };

     println!("The result is {result}");
}
