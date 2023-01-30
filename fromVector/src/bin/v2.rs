fn main(){
    let mut v = vec![1,2,3,100];

    for i in &mut v{
        *i += 50;
    }

    for i in v{
        println!("{}",i);
    }
}