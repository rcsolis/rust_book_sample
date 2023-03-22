
// Declare lifetime 'a in the function signature
// the function takes two parameters, 
// both of which are string slices that 
// live at least as long as lifetime 'a
// The function signature also tells that the string
// slice returned from the function will live at least as long as lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// Lifetime annotations in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn play_with(){
    println!("Playing with lifetimes");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("ImportantExcerpt: {}", i.part);
}