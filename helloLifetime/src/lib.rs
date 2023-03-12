#[cfg(test)]
mod lifetime {
    #[test]
    fn first_good_example() {
        // struct Point{
        //     des:&str,
        // }
        struct Point<'a> {
            des: &'static str,
            a: &'a str,
        }

        impl<'a> Point<'a> {
            fn level(&self) -> i32 {
                3
            }
        }

        //'a:'b表示‘a比‘b活得久
        //所以声明返回值为'b生命周期能引用生命为'a生命周期的self
        impl<'a: 'b, 'b> Point<'a> {
            fn haihai(&'a self, haihai: &'b str) -> &'b str {
                self.des
            }
        }
    }
    /*
    生命周期消除规则
    1.每个引用参数都会获得独自的生命周期
    2.若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期
    3.若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期

     */

    #[test]
    fn basic_end() {
        use std::fmt::Display;
        fn longest<'a, T>(x: &'a str, y: &'a str, pan: T) -> &'a str
        where
            T: Display,
        {
            eprintln!("{}", pan);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
}
