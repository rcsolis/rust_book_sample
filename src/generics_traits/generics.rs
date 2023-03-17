struct Point<T> {
    x: T,
    y: T,
}
struct Location<T,U>{
    lat: T,
    long: U,
}
impl<T, U> Location<T,U> {
    fn get_lat(&self) -> &T {
        &self.lat
    }
    fn get_long(&self) -> &U {
        &self.long
    }

    fn mixup<V, W>(self, other: Location<V, W>) -> Location<T, W> {
        Location {
            lat: self.lat,
            long: other.long,
        }
    }
}




fn create_structs(){
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("Point with integer coordinates: ({}, {})", integer.x, integer.y);
    println!("Point with float coordinates: ({}, {})", float.x, float.y);
    let location_ponts = Location { lat: 10, long: 20.0 };
    println!("Location with integer and float coordinates: ({}, {})", location_ponts.lat, location_ponts.long);
    println!("Get lat: {} Get Long: {}", location_ponts.get_lat(), location_ponts.get_long());
    let location_ponts2 = Location { lat: 10.0, long: "180.45" };
    println!("Location with float and str coordinates: ({}, {})", location_ponts2.lat, location_ponts2.long);
    let new_location = location_ponts.mixup(location_ponts2);
    println!("New location with integer and str coordinates: ({}, {})", new_location.lat, new_location.long);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn play_with(){
    println!("Generics funcitons");
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];
    let float_list = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = largest(&number_list);
    println!("The largest number in {:?} is {}", number_list, result);
    let result = largest(&char_list);
    println!("The largest char in {:?} is {}", char_list, result);
    let result = largest(&float_list);
    println!("The largest float in {:?} is {}", float_list, result);

    println!("Generics structs");
    create_structs();
}