use std::fmt::{Display, Debug};

fn main() {
    // let x = '1';
    // let y = "1211";
    // let z =String::from("1211");
    // let m = z+y;
    // let xx = 1221;

    // // print!("{}",z);
}

pub trait Summary {
    fn sumarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn sumarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: String,
}

impl Summary for Tweet {
    fn sumarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

pub trait Summaryy {
    // fn sumarize(&self)->String;
    fn sumarize(&self) -> String {
        String::from("default")
    }
}

//trait作为参数
//参数可以传Tweet或者Summaryy
pub fn notify(item: impl Summary) {
    println!("breaking news is {}", item.sumarize());
}

//trait bound 写法
//上述写法实际上是bound的语法糖 
pub fn notify1<T:Summary>(item: T) {
    println!("breaking news is {}", item.sumarize());
}

//指定多个trait要使用+

pub fn notify2<T:Summary + Display>(item: T) {
    println!("breaking news is {}", item.sumarize());
}

pub fn notify3(item: impl Summary+Display) {
    println!("breaking news is {}", item.sumarize());
}


//使用where

// pub fn notify4<T:Summary + Display,U:Clone+Debug>(item: T,utem:U) {
//     println!("breaking news is {}", item.sumarize());
// }


pub fn notify4<T,U>(item: T,utem:U) 
where
    T:Summary+Display,
    U:Clone+Debug, 

{
    println!("breaking news is {}", item.sumarize());
}


//函数返回类型也可以是trait
//但要注意的是
//函数返回的具体类型只能是一个



