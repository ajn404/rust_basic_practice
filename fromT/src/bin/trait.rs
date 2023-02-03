fn main(){
    // let x = '1';
    // let y = "1211";
    // let z =String::from("1211");
    // let m = z+y;
    // let xx = 1221;
    
    // // print!("{}",z);

    
}

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
    pub reply:String,
    pub retweet:String,
} 

impl Summary for Tweet {
    fn sumarize(&self)->String {
        format!("{}:{}",self.username,self.content)
    }
}


pub trait Summaryy {
    // fn sumarize(&self)->String;
    fn sumarize(&self)->String{
        String::from("default")
    }

}