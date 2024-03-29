fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;

    // can't do this as the condition must be a boolean not an int
    // if number {
    //   println!("number was three");
    // }

    if number != 0 {
      println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if is an expression you can use it in a statement with let
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    // "break" and "continue" break inner most loops in nested loops

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
              break;
            }
            if count == 2 {
              break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }    
    println!("End count = {count}");

    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
