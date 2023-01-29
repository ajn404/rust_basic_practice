
fn main(){
    let mut s = String::from("hello world");
    let first_index = first_index(&s);
    s.clear();
    println!("{}",first_index);
}

fn first_index(s:&String)->usize{
    let byte = s.as_bytes();
    for (i,&item) in byte.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}