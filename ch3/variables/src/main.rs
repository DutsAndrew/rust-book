fn main() {
  let mut x = 5;
  println!("The value of x is: {x}");

  x = 6;
  println!("The value of x is: {x}");

  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

  let x = x + 1;

  {
    let x = x * 2;
    println!("The value of x in the inner scope is {x}");
  }

  println!("the value of x is: {x}");

  let spaces = "   ";
  let spaces = spaces.len();

  let a = 2.0; // f64 type
  let b: f32 = 3.0; // f32

  // addition
  let sum = 5 + 10;

  //subtraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // results in -1
  let remainder = 43 % 5;

  // boolean
  let t = true;
  let f: bool = false;

  // char
  let c = 'z';
  let z: char = 'Z';
  let heart_eyed_cat = '😻';

  // tuple
  let tup: (i32, f64, u8) = (500, 6.4, 1);

  // destructured tuple
  let (x, y, z) = tup;
  println!("The value of y is: {y}"); // 6.4

  // can also access tuple elements with . notation
  let next_tup: (i32, f64, u8) = (500, 6.4, 1);
  let five_hundred = next_tup.0;
  let six_point_four = next_tup.1;
  let one = next_tup.2;

  // arrays, type is [type, length of array]
  let array_one: [i32, 5] = [1, 2, 3, 4, 5];

  // can also init an array with certain values
  let array_two = [3; 5]; // a = [3, 3, 3, 3, 3];

  // accessing array elements
  let array_three = [1, 2, 3, 4, 5];
  let first_element = a[0];
  let second_element = a[1];

  // will get an error if you try to index an array with an index out of bounds. Rust is unique in the sense it will not allow to check memory of an invalid index unlike other low level languages.
}
