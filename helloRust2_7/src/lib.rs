struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct Rectangle{
    width:u32,
    height:u32,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_num() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn impl_keyword() {
        impl Circle {
            /*
            这种定义在 impl 中且没有 self 的函数被称之为关联函数： 
            因为它没有 self，不能用 f.read() 的形式调用，
            因此它是一个函数而不是方法，它又在 impl 中，与结构体紧密关联，因此称为关联函数。
             */
            fn new(x: f64, y: f64, _: f64) -> Circle {
                Circle {
                    x: x,
                    y: y,
                    radius: 1.0,
                }
            }

            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }
    }

    #[test]
    //self &self &mut self
    fn lazy_name() {
        impl Rectangle{
            //方法名和结构体的名可以相同，根据调用的方式区分
            fn area(&self)->u32{
                self.width*self.height
            }

            fn new(width:u32,height:u32)->Self{
                Rectangle{width,height}
            }

        }
        let rec = Rectangle{width:100,height:100};
        // let rec1 = Rectangle::new(100, 200);                                                                    `
        let area = Rectangle::area(&rec);
        
        // let self_area = rec.area();
        let self_area = (&rec).area();
    
        dbg!([area,self_area]);
    }


    #[test]
    fn enum_impl(){
        #[derive(Debug)]
        enum Message{
            Quit,
            Move{x:i32,y:i32},
            Write(String),
            ChangeColor(i32,i32,i32),
        }

        impl Message{
            fn call(&self){
                dbg!(self);
            }

            pub fn quit()->Message{
                let msg = Message::Quit;
                msg
            }
        }
        let q =  Message::quit();
        dbg!(q);
    }

}
