fn match_fn(val:i32){
    match val{
        1=>println!("one"),
        2=>println!("two"),
        3=>println!("three"),
        _=>println!("{:?}",val)
    }
}



fn main(){
    let val =  4;
    match_fn(val);
    match_fn(1);
}