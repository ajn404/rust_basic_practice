fn main(){
    let mut i = 4;
    loop{
        println!("{:?}",i);
        i = i -1;
        if i < 1 {
            break;
        }

    }
    println!("done")
}