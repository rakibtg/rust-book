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

     // Loop labels: In a nested loop setup break and continue can be called for parent loops by it's label.
     // Important: Loop labels must begin with a single quote.
     let mut count = 0;
     'counting_up: loop {
         println!("count = {count}");
         let mut remaining = 10;

         loop {
             println!("remaining = {remaining}");

             if remaining == 9 { break; }
             if count == 2 { break 'counting_up; }

             remaining -= 1;
         }

         count += 1;
     }

     // While loop or conditional loop:
     let mut w_number = 3;
     while w_number != 0 {
         println!("{w_number}");
         w_number -= 1;
     }
     println!("While loop has ended.");

     // for loop to iterate over an array element.
     let an_array = [10, 20, 30, 40, 50];
     for element in an_array{
         println!("The value is: {element}");
     }
     for rev_num in (1..5).rev() {
         println!("{rev_num}");
     }
}
