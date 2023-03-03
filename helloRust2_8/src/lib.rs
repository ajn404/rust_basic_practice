#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[cfg(test)]
mod testtttt {

    use super::*;
    #[test]
    fn about_generics() {
        fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T
// where
        // T:std::cmp::PartialOrd
        {
            let mut res = list[0];
            for &item in list.iter() {
                if item > res {
                    res = item
                }
            }
            res
        }

        let number_list = vec![1, 2, 3, 4];
        let res = largest(&number_list);
        dbg!(res);
    }

    #[test]

    fn struct_generics() {
        let int = Point { x: 1, y: 2 };
        let flo = Point { x: 1.1, y: 1.2 };
        dbg!((&int, &flo));
    }

    #[test]
    fn struct_generics_different() {
        #[derive(Debug)]
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let x = Point { x: 1, y: 2.2 };
        dbg!(x);
    }

    #[test]
    fn dragon_enum_phoenix_generics() {
        enum Option<T> {
            Some(T),
            None,
        }

        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }

    #[test]
    fn impl_generics() {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        let p = Point { x: 20, y: 20 };
        eprintln!("p.x() = {}", p.x());
    }

    //在结构体方法中定义额外的范型参数
    #[test]
    fn lazy_name_generics() {
        #[derive(Debug)]
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 1, y: 2 };

        let p2 = Point { x: 3, y: 4 };

        let p3 = p1.mixup(p2);
        dbg!(p3);
    }

    #[test]
    fn specific_type() {
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let p1: Point<f32> = Point { x: 1.11, y: 2.22 };
        dbg!(p1.distance_from_origin());
    }

    // #[test]
    // fn const_something() {
    //     #![allow(incomplete_features)]

    //     pub enum Assert<const CHECK: bool> {
    //         //
    //     }

    //     pub trait IsTrue {
    //         //
    //     }

    //     impl IsTrue for Assert<true> {
    //         //
    //     }

    //     fn something<T>(val: T)
    //     where
    //         Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
    //         //       ^-----------------------------^ 这里是一个 const 表达式，换成其它的 const 表达式也可以
    //     {
    //         //
    //     }

    //     something([0u8; 0]);
    //     // ---

        
    // }

    


}
