pub trait Summary {
    fn sumarize(&self)->String;
}

pub struct NewsArticle{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

impl Summary for NewsArticle {
    fn sumarize(&self)->String {
        format!("{} by {} ({})",self.headline,self.author,self.location)
    }       
}


pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
} 

impl Summary for Tweet {
    fn sumarize(&self)->String {
        format!("{}:{}",self.username,self.content)
    }
}