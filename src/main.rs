
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        match self {

            Message::ChangeColor(a,b,c)=> println!("I'm a color change!"),
            Message::Write(r)=> println!("Write {r}"),
            Message::Move{x,y} => println!("Movin' on up"),
            Message::Quit => println!("done!")
         }
    }
}


fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn add(a: u32, b: u32) -> u32 {

    return a + b;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
