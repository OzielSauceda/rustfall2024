use std::fs::File;
use std::io::{self, Read, Write};

struct Car {
    name: String,
    model: u32,
}

fn reading_from_console() -> Car{
    let mut buffer = String::new();

    print!("What's your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("What model is it? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model: u32 = buffer.trim().parse().expect("Please enter a valid model.");

    Car { name, model }
}

fn save_to_file(car: &Car){
    let mut file = File::create("user_info.txt").unwrap();
    writeln!(file, "Car: {}", car.name).unwrap();
    writeln!(file, "Model: {}", car.model).unwrap();
}

fn read_from_file(){
    let mut file = File::open("user_info.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("File content:\n{}", content);
}

fn main() {
    let car = reading_from_console();

    save_to_file(&car);
    println!("Car information was saved to the file.");

    read_from_file();
}