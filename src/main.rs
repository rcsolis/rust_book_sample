

fn main() {
    // Primitive data types
    let x: i32 = 5;
    let y: f32 = 5.0;
    let z: bool = true;
    let a: char = 'z';
    let b: &str = "Hello, world!";

    println!("{b}");
    println!("Variables are immutable by default.");
    println!("x = {x}, y={} a={a} ", y * 2.0);
    // Using if
    if z {
        println!("z is true");
    } else {
        println!("z is false");
    }
    guess_number_game();
}
// Implementing a guessing number game
fn guess_number_game(){
    use std::cmp::Ordering;
    use std::io;
    const UPPER_LIMIT: u32 = 100;
    let random_number:u32 = random_number(UPPER_LIMIT);
    loop{
        println!("Enter a number between 1 and {UPPER_LIMIT}: ");
        let mut guess = String::new();
        // Get user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Shadowing, parse the user input to a number and check if it is a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("The secret number is: {}", random_number);
}
// Random number generator
fn random_number(upper:u32) -> u32 {
    use rand::Rng;
    let secret_number = rand::thread_rng().gen_range(1..=upper);
    return secret_number;
}
