#[derive(Debug)]

struct Rectangle{
    width:u32,
    height:u32,
}

fn area(rect:&Rectangle)->u32{
    rect.height * rect.width
}


fn main(){
    let rect = Rectangle{
        width:30,
        height:30,
    };
    println!("{:?}",area(&rect));
    println!("{:#?}",&rect);
     

}