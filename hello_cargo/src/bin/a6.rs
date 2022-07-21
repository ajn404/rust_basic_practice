fn main(){
    let mut x = 99;
    while x>10{
        x = x - 2;
        println!("{:?}=>{:?}",x,x-2);
    }
    println!("done");
}