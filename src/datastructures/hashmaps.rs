use std::collections::HashMap;
pub fn play_with(){
    println!("Play with hashmaps!");
    // Hashmaps are unordered
    let mut css_colors = HashMap::new();
    // Add values
    css_colors.insert("Blue", (0,0,255));
    css_colors.insert("Red", (255,0,0));
    css_colors.insert("Green", (0,255,0));
    css_colors.insert("White", (0,0,0));
    css_colors.insert("Yellow", (255,0,0));
    // Get value
    let color = "Red";
    let rgb_color = css_colors.get(color).copied().unwrap_or((0,0,0));
    println!("RGB of {color} {:?}", rgb_color);
    // Loop
    for (k,v) in &css_colors{
        println!("Color: {k} Value:{:?}", v);
    }
    // Overwriting values
    css_colors.insert("White", (255,255,255));
    // Insert if not exists
    css_colors.entry("Black").or_insert((0,0,0));
    css_colors.entry("Yellow").or_insert((255,255,255));
    // Loop
    println!("Current values:");
    for (k,v) in &css_colors{
        println!("Color: {k} Value:{:?}", v);
    }
    // Update using the current value
    {
        let rgb = css_colors.entry("Yellow").or_insert((0,0,0));
        (*rgb).1 = 255;
    }
    println!("Last values:");
    for (k,v) in &css_colors{
        println!("Color: {k} Value:{:?}", v);
    }
    // Counter sample
    let mut counter_map = HashMap::new();
    for w in "Rust is easy, Rust is clean, Rust rules!".split_whitespace() {
        // Return reference if exists or initialize with 0 and return a reference
        let count = counter_map.entry(w).or_insert(0);
        // Update value
        *count += 1;
    }
    println!("Word counter for 'Rust is easy, Rust is clean, Rust rules!'");
    println!("{:?}", counter_map);
}