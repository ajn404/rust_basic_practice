

#[derive(Debug)]
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn Call(&self){

    }
}

fn main(){
    let q = Message::Quit;
    let m = Message::Move { x: 100, y: 200 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(23, 23, 23);
    println!("{:?}\n{:?}\n{:?}\n{:?}",q,m,w,c);

    q.Call();
    println!("ok");
}