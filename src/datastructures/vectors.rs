pub fn play_with(){
    println!("Play with vectors!");
    // Declare mutable vector to add new values
    let mut vec: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1,10,100];
    // Add values
    vec.push(1);
    vec.push(2);
    vec2.push(1000);
    // Get values by reference
    let n = &vec2[3];
    println!("Value of vector2 at 3={}", n);
    // Get values using get method
    if let Some(n) = vec.get(1) {
        println!("Get value with get method:{}", n);
    }
    let n = match vec.get(4) {
        Some(n) => n,
        None => {
            println!("Element at 4 does not exists");
            &0
        }
    };
    println!("Value of vector at 4={}", n);
    
    // Update values using dereferencing
    for i in &mut vec {
        *i +=200;
    }
    for i in &vec {
        println!("New value: {i}");
    }
}