// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(u32),
    Echo,
    Move,
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit(1));
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
