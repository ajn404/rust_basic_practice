use std::io;

use std::cmp::Ordering;
use rand::Rng;
fn main(){
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("the secret number is {}",secret_number);



    loop{
        println!("please input your guess :");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        println!("Your guess is :{}",guess);
        //将输入的字符串转成数字
        //u32 32位无符号整数
        let guess:u32=guess.trim().parse().expect("please type a number");
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too small!"),
            Ordering::Greater=>println!("Too big"),
            Ordering::Equal=>{
                println!("you win!");
                break;
            }
        }
    }


}