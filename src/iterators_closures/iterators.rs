

pub fn play_with(){
    println!("Play with iterators!");

    let vec = vec![1, 2, 3, 4, 5];

    println!("Use iterator");
    let iter1 = vec.iter();
    for i in iter1 {
        println!("i: {}", i);
    }
    
    println!("Sum of vec using iterator: {}", vec.iter().sum::<i32>());
    println!("Adds two to each element of vec");
    let vec2:Vec<_> = vec.iter().map(|x| x + 2).collect();
    println!("vec1: {:?} vec2: {:?}", vec, vec2);
}