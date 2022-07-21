enum Direction{
    Up,Down,Left,Right
}

fn which_way(go: Direction){
    match go{
        Direction::Up=>println!("Up"),
        Direction::Down=>println!("down"),
        Direction::Left=>println!("Left"),
        Direction::Right=>println!("right"),
    }
}


fn main(){
    which_way(Direction::Left);
    which_way(Direction::Right);
    which_way(Direction::Up);
    which_way(Direction::Down);
}