use std::io;
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
    
    println!("Play with arrays and tuples!");
    println!("Tuples are immutable by default and always store the same data types.");
    let dataypes_tuple: (i32, f32, bool, char) = (1, 2.0, true, 'a');
    println!("dataypes_tuple = {:?}", dataypes_tuple);
    println!("dataypes_tuple.0 = {}", dataypes_tuple.0);
    println!("dataypes_tuple.2 = {}", dataypes_tuple.2);
    let (x, y, z, a) = dataypes_tuple;
    println!("Destructuring: x = {}, y = {}, z = {}, a = {}", x, y, z, a);
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Arrays are like tuples but all elements are the same data type.");
    println!("array[2] = {}, len={}", array[2], array.len());
    let array_initialized_same_value = [0; 5];
    println!("array2[2] = {}, len={}", array_initialized_same_value[2], array_initialized_same_value.len());

    println!("Play with loops!");
    test_loops();

    println!("Play with slices!");
    test_slices();

    structs_sample();

    println!("Play with enums!");
    enums_sample();
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
fn test_loops(){
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

fn test_slices(){
    println!("Slice is a reference to a part of a string, also a string literal is a slice");
    let s = String::from("hello world");
    let hello = &s[0..5]; // This is a slice
    let world = &s[6..11]; 
    println!("hello = {}, world = {}", hello, world);
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    println!("slice = {}", slice);
    let slice = &s[..]; // works the same as &s[0..len]
    println!("slice = {}", slice);
    let s = String::from("hello world");
    let word = get_first_word(&s);
    println!("word = {}", word);
    let my_string_literal = "hello world";
    let word = get_first_word(&my_string_literal[..]);
    println!("word = {}", word);
    let word = get_first_word(my_string_literal);
    println!("word = {}", word);
    let word_slice = "hello world";
    let word = get_first_word(word_slice);
    println!("word = {}", word);
    println!("Slices can be used with arrays too");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice = {:?}", slice);
}
// String literals are slices
fn get_first_word(phrase: &str) -> &str{
    let bytes = phrase.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &phrase[0..i];
        }
    }
    &phrase[..]
}

// Structs
struct Record{
    epic: String,
    pam: String,
    duration:u8,
    date: String,
}
struct TupleField(u8, String,f32,bool);
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}
//Methods for rectangle
// Methods of Rectangle
impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rectangle:&Rectangle) -> bool{
        //self.width  > rectangle.width && self.height > rectangle.height
        self.area() > rectangle.area()
    }
    // Associated functions (like static in other language)
    // Common used as "constructors" or static class methods
    fn square(size: u32)-> Self {
        Self{
            width:size,
            height:size
        }
    }
}

fn build_record(epic:&str, pam: &str, date:&str, duration:u8) -> Record{
    Record{
        epic: String::from(epic),
        pam: String::from(pam),
        date: String::from(date),
        duration,
    }
}
fn structs_sample() {
    println!("Play with structs!");
    let row: Record = build_record("CAMTAC-9090", "Allan Calvo", "01/03/2023", 4);
    let row2 = Record{
        date: String::from("02/03/2023"),
        pam: String::from("JuanJose"),
        epic: String::from("CAMECOM-8989"),
        ..row
    };
    println!("Record 2: {} ({}) Dur:{} PAM:{}", row2.epic, row2.date, row2.duration, row2.pam);
    println!("the row variable still be accessible after update syntax because we update all the fields that implement COPY trait");
    println!("Record: {} ({}) Dur:{} PAM:{}", row.epic, row.date, row.duration, row.pam);
    let row3 = Record{
        duration:2,
        ..row
    };
    println!("Now, because we use the update syntax with values that dont has COPY trait, tha data was move so row is no longer available!!");
    println!("Record 3: {} ({}) Dur:{} PAM:{}", row3.epic, row3.date, row3.duration, row3.pam);
    
    println!("Tuple String are useful whan we want to create a named tuple to reuse as a type");
    println!("We can use the .# syntax to access the field or by destructuring!");
    let trow = TupleField(10, String::from("HOLA"),89.99,false);
    let TupleField(f1,f2,f3,f4) = trow;
    println!("Fields of Tuple Struct: {} {} {} {}", f1,f2,f3,f4);
    
    println!(" Using methods");
    let rect1 = Rectangle{
        width: 100,
        height: 8,
    };
    let rect2 = Rectangle {
        width: 90,
        height: 7,
    };
    let rect3 = Rectangle {
        width: 600,
        height: 5,
    };
    
    println!("Rectangle (with derived Debug): {:?}", rect1);
    dbg!(&rect1);
    println!("Call method area for rect1: {}", rect1.area());
    println!("Call method area for rect2: {}", rect2.area());
    println!("Call method area for rect3: {}", rect3.area());
    
    println!("Rect2 inside of rect1: {}", rect1.can_hold(&rect2));
    println!("Rect3 inside of rect1: {}", rect1.can_hold(&rect3));
    let square = Rectangle::square(50);
    println!("Square created with associated function: {}", square.area());
}

// ENUMS
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct MyIpAddress{
    kind: IpAddrKind,
    address: String,
}
#[derive(Debug)]
enum Developer{
    SR(String),
    JR(String),
}
#[derive(Debug)]
enum CSSColor{
    RGB(u8,u8,u8),
    HEX(String),
}

enum Currency{
    Dollar,
    Euro,
    Colon
}

fn peso_convertion_rate(curr: Currency) -> i32 {
    match curr {
        Currency::Dollar => 20,
        Currency::Euro => 22,
        Currency::Colon => 5,
        _ => 1,
    }
}
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    Dallas,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Dollar
}

fn value_in_cents(coin: Coin) -> u8{
    
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from the state: {:?}", state);
            25
        }
        _=>{
            println!("Use catch-all pattern to satisfy the exhaustiveness principle for match");
            println!("You can use _ if don't want to use the value else you can define a variable name.");
            100
        }
    }
}

fn add_one_to(number:Option<u8>) -> Option<u8> {
    match number {
        None => {
            println!("Empty Value! Like null");
            None
        }
        Some(n) => Some(n+1),
    }
}

fn enums_sample(){
    let localhost = MyIpAddress {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("First Enum for ip address without data associated: {:?}",localhost);
    let sr_rust = Developer::SR(String::from("Rust"));
    let jr_go = Developer::JR(String::from("GO"));
    println!("Associated data with an enum (Same type)");
    println!("SR: {:?} JR:{:?}", sr_rust, jr_go);
    let black = CSSColor::HEX(String::from("#000"));
    let red = CSSColor::RGB(255,0,0);
    println!("Associated data with an enum (Different types)");
    println!("HEX: {:?} RGB:{:?}", black, red);
    
    let some_number: Option<u8> = Option::Some(255);
    let null_number: Option<u8> = Option::None;
    
    println!("Use standard Option<T> type:");
    println!("Not null: {:?} null:{:?}", some_number, null_number);
    
    println!("Play with MATCH again!");
    let peso_usd = peso_convertion_rate(Currency::Dollar);
    println!("Peso to USD: {}", peso_usd);
    println!("Patterns with Enums (Returns value)");
    let cents = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("Alabama quarter returns: {}", cents);
    let cents = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Alaska quarter returns: {}", cents);
    let cents = value_in_cents(Coin::Quarter(UsState::Dallas));
    println!("Dallas quarter returns: {}", cents);
    println!("We need to define all possible patterns when use match");
    let cents = value_in_cents(Coin::Dollar);
    println!("Dollar:{}", cents);
    println!("Match with Option<T>");
    
    let five:Option<u8> = Some(5);
    let six = add_one_to(five);
    println!("{:?} + 1 = {:?}", five, six);
    let no_value = add_one_to(Option::None);
    println!(" Null:{:?}", no_value);
}