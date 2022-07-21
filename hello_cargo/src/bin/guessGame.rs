use std::io;
fn main(){
    println!("guess your number!");
    println!("Please inpput your guess number:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Falied to read line");
    println!("your guess:{}",guess);
}