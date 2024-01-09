fn main() {
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1 // no semi-colon as this is an expression, statements end with semicolons
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let sum = plus_one(5);
    println!("The value of sum is: {sum}");    
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // no semi colon as it's an expression
}