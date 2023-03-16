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

pub fn play_with(){
    println!("Play with ENUMS");
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