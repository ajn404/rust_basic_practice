#[derive(Debug)]

struct Rectangle{
    width:u32,
    height:u32,
}
//定义方法
impl Rectangle {
    fn area(&self)->u32{
        self.width*self.height
    }

    
}

fn area(rect:&Rectangle)->u32{
    rect.height * rect.width
}




fn main(){
    let rect = Rectangle{
        width:30,
        height:30,
    };
    println!("area(&rect){:?}",area(&rect));
    println!("rect.area(){:?}",rect.area());
    println!("{:#?}",&rect);
     

}