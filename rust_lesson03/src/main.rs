fn main() {
    // 枚举
    enum Message {
        Quit,
        changeColor(i32, i32, i32),
        Move {x: i32, y: i32},
        Write(String),
    }
    let x: Message = Message::Move{x: 3, y: 5};
    println!("Hello, world!");
}
