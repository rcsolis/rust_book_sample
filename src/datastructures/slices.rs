pub fn play_with(){
    println!("Play with slices!");
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