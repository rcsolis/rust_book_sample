pub fn play_with(){
    println!("Play with arrays and tuples!");
    println!("Tuples are immutable by default and always store the same data types.");
    let datatypes_tuple: (i32, f32, bool, char) = (1, 2.0, true, 'a');
    println!("datatypes_tuple = {:?}", datatypes_tuple);
    println!("datatypes_tuple.0 = {}", datatypes_tuple.0);
    println!("datatypes_tuple.2 = {}", datatypes_tuple.2);
    let (x, y, z, a) = datatypes_tuple;
    println!("Destructuring: x = {}, y = {}, z = {}, a = {}", x, y, z, a);
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Arrays are like tuples but all elements are the same data type.");
    println!("array[2] = {}, len={}", array[2], array.len());
    let array_initialized_same_value = [0; 5];
    println!("array2[2] = {}, len={}", array_initialized_same_value[2], array_initialized_same_value.len());

}