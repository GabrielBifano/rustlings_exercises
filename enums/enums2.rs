// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
enum Message {
    Move {x:i32, y:i32},
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
    fn Add(a:i32,  b:i32) -> i32{
        a + b
    }
}

fn main() {
    let messages = [    
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
        ];
        
    for message in &messages {
        message.call();
    }

    println!("{:?}", Message::Add(3, 55));
}
