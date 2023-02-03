use fromT::Summary;
use fromT::Tweet;

fn main() {
    let tweet = Tweet{
        username:String::from("username"),
        content:String::from("content :我打算寄快递把升级换代v啊世界的v啊睡觉"),
        reply:false,
        retweet:true,
    };
    println!("tweet format 后为:\n{}",tweet.sumarize());
}
