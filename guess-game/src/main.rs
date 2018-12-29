extern crate rand;

use std::io;
use std::cmp::Ordering; // an enum with values Less, Grater & Equal
use rand::Rng;

fn main() {
  let cat = "😻";

  println!("{}", cat);
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  // println!("The secret number is: {}", secret_number);

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    // * `read_line` takes whatever the user types into standard input and place
    // that into a string. The & indicates that this argument is a reference.
    // * `read_line` also returns an `io::Result` (enum) instance.
    // * If this instance of io::Result is an Err value, expect will cause the
    // program to crash and display the message that you passed as an argument.
    // * If this instance of io::Result is an Ok value, expect will take the
    // return value that Ok is holding and return just that value to you.
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    // shadow the previous value of guess with a new one..
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => { // The underscore, _, is a catchall value (match all Err)
        println!("not a number...");
        continue;
      }
    };

    println!("You guessed: {}", guess);

    // * cmp method compares two values and can be called on anything that can be
    // compared.
    // * match is like a switch.
    match guess.cmp(&secret_number) {
      // all statements bellow are called "arms"
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You WON!");
        break; // exit the loop
      }
    }
  }
}
