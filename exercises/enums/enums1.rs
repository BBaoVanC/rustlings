// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor { r: u8, g: u8, b: u8 },
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("abcdefg")));
    println!("{:?}", Message::Move{ x: 123, y: 456 });
    println!("{:?}", Message::ChangeColor{ r: 0, g: 127, b: 255 });
}
