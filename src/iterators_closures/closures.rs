#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
    Green,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            shirts: vec![ShirtColor::Red, 
            ShirtColor::Blue,
            ShirtColor::Green,
            ShirtColor::Blue],
        }
    }

    fn giveaway( &self, 
        user_preference: Option<ShirtColor>) -> ShirtColor{
            user_preference.unwrap_or_else(|| {
                self.most_stocked_color()
            })
    }

    fn most_stocked_color(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        let mut num_green = 0;
        
        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Green => num_green += 1,
            }
        }
        println!("STOCK= Red: {}, Blue: {}, Green: {}", 
            num_red, num_blue, num_green); 
        if num_red > num_blue && num_red > num_green {
            ShirtColor::Red
        } else if num_blue > num_red && num_blue > num_green {
            ShirtColor::Blue
        } else {
            ShirtColor::Green
        }

    }
}

pub fn play_with() {
    println!("Play with closures!");
    let inventory = Inventory::new();
    let user_pref = Some(ShirtColor::Green);
    let shirt = inventory.giveaway(user_pref);
    println!("Shirt preference {:?} gets color: {:?}", 
        user_pref, 
        shirt);
    let user_pref2 = None;
    let shirt2 = inventory.giveaway(user_pref2);
    println!("Shirt preference {:?} gets color: {:?}", 
        user_pref2, 
        shirt2);
}