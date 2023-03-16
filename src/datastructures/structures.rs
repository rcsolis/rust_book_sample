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
pub fn play_with() {
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