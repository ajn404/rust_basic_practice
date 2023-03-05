#[cfg(test)]
pub struct Post {
    pub title: String,
    pub auther: String,
    pub content: String,
}

pub trait Summry {
    fn summerize(&self) -> String {
        String::from("more infomation")
    }
}
mod test {
    use super::*;
    use std::fmt::{format, Display};
    #[test]
    fn add() {
        fn add<T: std::ops::Add<Output = T>>(a: T, b: T) {}
    }

    #[test]
    fn define_trait() {
        pub trait Sun {
            fn sum(&self) -> String {
                dbg!("trait");
                "hhh".to_string()
            }

            fn not_imp() -> str;
            fn not_imp_and_str() -> &'static str;
            fn not_imp_and_str_not_static<'a>(a: &'a str) -> &'a str;
        }
    }

    #[test]
    fn impl_trait() {
        pub trait Summry {
            fn summerize(&self) -> String;
        }

        impl Summry for Post {
            fn summerize(&self) -> String {
                format!("文章{}的作者是{}", self.title, self.auther)
            }
        }

        let po = Post {
            title: "关于******的文章".to_string(),
            auther: "ajn404".to_string(),
            content: "这是一个晴朗的周天，非常的nice".to_string(),
        };

        let sum = po.summerize();
        dbg!(sum);
    }

    //孤儿规则
    //如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！
    //例如你无法在当前作用域中，为 String 类型实现 Display 特征，因为它们俩都定义在标准库中，其定义所在的位置都不在当前作用域
    #[test]
    fn orphanRule() {}

    //默认实现
    #[test]
    fn default_impl() {
        #[derive(Debug)]
        struct Post {
            pub title: String,
            pub auther: String,
        }

        #[derive(Debug)]
        struct Default_impl {}
        impl Summry for Post {
            fn summerize(&self) -> String {
                format!("作者是{} 文章是{}", self.auther, self.title)
            }
        }
        impl Summry for Default_impl {}
        let x = Post {
            title: "哈hi啊hi啊".to_string(),
            auther: "ajn440".to_string(),
        };

        let y = Default_impl {};
        dbg!(&x);
        dbg!(&y);
        eprintln!("{}", x.summerize());
        eprintln!("{}", y.summerize());
    }

    #[test]
    fn default_impl_plus() {
        pub trait Summry {
            fn sum(&self) -> String;

            fn summerize(&self) -> String {
                format!("more infomation {}", self.sum())
            }
        }
        //为实现上述特征，只需实现sum函数即可，
        //调用summerize会间接调用实现的sum函数
    }

    #[test]
    //特征作为函数参数
    fn trait_as_params() {
        pub fn notify(a: &impl Summry) {
            a.summerize();
        }
        //&impl 是特征约束的语法糖，是指实现了Summry特征的所有类型

        pub fn _notify<T: Summry>(a: &T) {
            a.summerize();
        }
    }

    #[test]
    fn multi_bounds() {
        pub fn notify(a: &(impl Summry + Display)) {
            a.summerize();
        }

        pub fn _notify<T: Summry + Display>(a: &T) {
            a.summerize();
        }
    }

    #[test]
    fn bounds_use_were() {
        pub fn notify<T: Display + Clone + Summry>(a: &T) -> T {
            let b = a.clone();
            b
        }

        pub fn _notify<T, U>(t: T, u: U)
        where
            T: Display + Clone + Summry,
            U: Display + Summry,
        {
        }
    }

    //使用特征约束有条件地实现方法或者特征
    #[test]
    fn conditional_impl_or_trait() {
        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Pair { x, y }
            }
        }

        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if (self.x > self.y) {
                    eprintln!("the larger number is {}", self.x);
                } else {
                    eprintln!("the larger number is {}", self.y);
                }
            }
        }

        //当且实现了Display和PartialOrd的Pair<T>才会有cmp_display方法
    }

    //函数返回中的特征约束

    #[test]
    fn function_return_bounds() {
        fn notify() -> impl Summry {
            struct x {}
            impl Summry for x {}

            let y = x {};
            y
        };

        //这种返回还有一个很大的限制，就是返回的实际类型只能有一个，即使返回的类型全都满足实现了特征
        //例如

        // fn _notify(toggle: bool) -> impl Summry {
        //     struct X {}
        //     impl Summry for X {}
        //     struct Y {}
        //     impl Summry for Y {}
        //     if toggle {
        //         let x = X {};
        //         x
        //     } else {
        //         let y = Y {};
        //         y
        //         /*
        //                         `if` and `else` have incompatible types
        //         expected struct `X`, found struct `Y`
        //                          */
        //     }
        // }
    }

    //derive 派生特征
    //被derive标记的对象会实现默认的特征代码
    


    //有时候调用方法需要引入特征
    #[test]
    fn use_trait_brfore_call_function(){
        use std::convert::TryInto;
        //TryInto本身就在prelude中
        let x =100;
        let b = 20;
        let b_ = b.try_into().unwrap();
        if x>b_{
            eprintln!("a is larger than b so that i don;t need to call upside function");
        }
    }

    //几个综合例子

    //为自定义类型实现假发操作
    #[test]
    fn example_1(){
        use std::ops::Add;
        #[derive(Debug)]
        pub struct Point<T:Add<T,Output = T>>{
            x:T,
            y:T,
        }
        impl <T:Add<T,Output = T>> Add for Point<T>{
            type Output = Point<T>;
            fn add(self, rhs: Self) -> Self::Output {
                Point{
                    x:self.x+rhs.x,
                    y:self.y+rhs.y,
                }
            }    
        }

        fn add<T:Add<T,Output = T>>(a:T,b:T)->T{
            a + b
        }

        let x = Point{x:1.123,y:2.111};
        let y = Point{x:9999.9,y:1.0};
        let x_plus_y = add(x,y);
        dbg!(x_plus_y);
    }
    

    //自定义类型的打印输出

    #[test]

    fn example_2(){
        //为自定义类型实现Display特征
        use std::fmt;
        use std::fmt::Display;


        #[derive(Debug,PartialEq)]
        enum FileState{
            Open,
            Closed,
        }


        #[derive(Debug)]
        struct File{
            name:String,
            data:Vec<u8>,
            state:FileState,
        }

        //impl Display for FileState

        impl Display for File{
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f,"<{},({:?})>",self.name,self.state)
            }
        }

        impl File{
            fn new(name:&str)->File{
                File { name:name.to_string(), data: Vec::new(), state:  FileState::Open}
            }
        }


        let f6 = File::new("new_file.txt");
        dbg!(&f6);
        eprintln!("{}",f6);
        eprintln!("{:?}",f6);

    }
}
