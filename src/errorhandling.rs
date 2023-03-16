pub mod unrecoverable_errors {
    use std::io;
    fn use_panic() {
        println!("Use panic! macro to stop the program");
        let mut use_p = String::new();
        println!("Do you want to PANIC? y/n");
        io::stdin()
            .read_line(&mut use_p)
            .expect("Failed to read line");
        match use_p.chars().next() {
            Some('y') => panic!("PANIC! This is an unrecoverable error."),
            Some('n') => println!("No panic"),
            _ => println!("No panic"),
        };

    }
    pub fn play_with() {
        println!("Play with unrecoverable errors!");
        use_panic();
    }
}


pub mod recoverable_errors {
    use std::io::{self, ErrorKind, Read, Write};
    use std::fs::{self, File};

    fn open_file() -> File{
        println!("Result enum type is used to handle recoverable errors");
        println!("Has two variants: Ok and Err");
        let file_name = "hello.txt";
        println!("Trying to open file: {}", file_name);
        println!("Handling different types of errors");
        let file_result = File::open(file_name);
        let file = match file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(file_name){
                    Ok(fc)=>{
                        println!("File {file_name} created");
                        fc
                    },
                    Err(err)=>panic!("Problem creating the file: {:?}", err),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            }
        };
        println!("File opened successfully");
        return file;
    }

    fn open_file_v2() -> File {
        let file_name = "hello.txt";
        println!("Open file and handle errors with closures");
        let file = File::open(file_name).unwrap_or_else(|error|{
            if error.kind() == ErrorKind::NotFound {
                File::create(file_name).unwrap_or_else(|error|{
                    panic!("Problem creating the file: {:?}", error);
                })
            }else{
                panic!("Problem opening the file: {:?}", error);
            }
        });
        return file
    }

    fn read_file(filename: &str) -> Result<String, io::Error> {
        let file_result = File::open(filename);
        let mut file = match file_result {
            Ok(file) => file,
            Err(error) => return Err(error),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => Ok(contents),
            Err(error) => Err(error),
        }
    }

    fn read_file_v2(filename: &str) -> Result<String, io::Error> {
        let mut contents = String::new();
        // Alternative: 
        // File::open(filename)?.read_to_string(&mut contents)?;
        let mut file = File::open(filename)?;
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn read_file_shorter(filename: &str) -> Result<String, io::Error> {
        fs::read_to_string(filename)
    }

    fn write_file(filename: &str, content: &str) -> Result<(), io::Error> {
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())?;
        return Ok(());
    }

    pub fn play_with(){
        println!("Play with recoverable errors!");
        open_file();
        open_file_v2();
        let wrong_file_name = "hello2.txt";
        let filename = "hello.txt";
        let file_content = "Hello, world!";
        println!("Writing to file: {}", filename);
        write_file(filename, file_content).expect("Error writing to file");
        let content = read_file(filename).expect("Error reading file");
        println!("Content of file v1: {}", content);
        match read_file_v2(wrong_file_name) {
            Ok(content) => println!("Content of file v2: {}", content),
            Err(error) => println!("Error reading file v2: {:?}", error),
        }
        println!("Content of file v3: {}", read_file_shorter(filename).expect("Error reading file v3"));
    }
}