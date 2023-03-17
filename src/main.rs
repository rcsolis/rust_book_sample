use std::io;

mod datastructures;
mod errorhandling;
mod generics_traits;

// Inline module
mod loops {
    pub fn play_with(){
        println!("Play with loops!");
        // loop with return value
        let mut counter = 0;
        println!("Loop with return value");
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {}", result);
        // Labeled loop
        counter = 0;
        println!("Labeled loop");
        'outer: loop {
            counter += 1;
            if counter == 3{
                break;
            }
            let mut counter2 = 0;
            println!("Entered the outer loop");
            loop {
                println!("Entered the inner loop");
                // This would break only the inner loop
                // break;
                // This breaks the outer loop
                counter2 += 1;
                if counter2 == 10 {
                    println!("Break outer loop from inner loop {} ", counter2);
                    continue 'outer;
                }
            }
        }
        // WHILE LOOPS
        println!("While loop");
        counter = 4;
        while counter != 0 {
            println!("counter in while: {}!", counter);
            counter -= 1;
        }
    
        // FOR LOOPS
        println!("For loop over an array");
        let array = [10, 20, 30, 40, 50];
        for element in array{
            println!("element in array: {}", element);
        }
    
        println!("For loop over a range");
        for number in (1..4).rev() {
            println!("Counting down {}!", number);
        }
    
    }
}
// This is the main function
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

    let mut want_play: String = String::new();
    println!("Do you want to play a guessing game? (y/n)");
    io::stdin()
        .read_line(&mut want_play)
        .expect("Failed to read line");
    let want_play: char = match want_play.chars().next() {
        Some('y') => 'y',
        Some('n') => 'n',
        _ => 'n',
    };
    if want_play == 'y'{
        guess_number_game();
    }
    
    datastructures::arrayandtuple::play_with();
    loops::play_with();
    datastructures::slices::play_with();
    datastructures::structures::play_with();
    datastructures::enumerations::play_with();
    datastructures::vectors::play_with();
    datastructures::hashmaps::play_with();
    
    println!("Error handling");
    errorhandling::unrecoverable_errors::play_with();
    errorhandling::recoverable_errors::play_with();

    println!("Generic Types, Traits and Lifetimes");
    generics_traits::generics::play_with();
}
// Implementing a guessing number game
fn guess_number_game(){
    use std::cmp::Ordering;
    
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
    // Is the same as: return secret_number;
    secret_number
}