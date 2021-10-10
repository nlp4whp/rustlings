// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(String),
    Echo,
    Move {x: i32, y: i32},
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit(String::from("hello Enum")));
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move {x: 1, y: 2});
    println!("{:?}", Message::ChangeColor);
}
