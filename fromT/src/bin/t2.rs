//函数范型
fn largetest<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }
    largest
}
//结构体范型

#[derive(Debug)]

struct Point<T> {
    x: T,
    y: T,
}

struct PPoint<T, U> {
    x: T,
    y: U,
}

//枚举反省、
//Option<T,E>之类的

//结构体的方法使用范型
impl<T> Point<T> {
    fn getX(&self) -> &T {
        &self.x
    }
}

//特定的Point<i32>才有getY这个方法
//针对具体的类型实现方法
impl Point<i32> {
    fn getY(&self) -> &i32 {
        &self.y
    }
}

impl<T, U> PPoint<T, U> {
    fn mixup<V, W>(self, other: PPoint<V, W>) -> PPoint<T, W> {
        PPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    let p1 = PPoint { x: 20, y: "dasdas" };
    let p2 = PPoint { x: "wolao", y: 12 };
    let p3 = p1.mixup(p2);
    print!("p3 x={:?}\ny={:?}\n", p3.x, p3.y);
    //x,y的类型相同

    let str_list = vec![String::from("hello "), String::from("world")];
    let res = largetest(&str_list);
    println!("{}", res);


    // impl<T: fmt::Display + ?Sized> ToString for T {
    //     // A common guideline is to not inline generic functions. However,
    //     // removing `#[inline]` from this method causes non-negligible regressions.
    //     // See <https://github.com/rust-lang/rust/pull/74852>, the last attempt
    //     // to try to remove it.
    //     #[inline]
    //     default fn to_string(&self) -> String {
    //         let mut buf = String::new();
    //         let mut formatter = core::fmt::Formatter::new(&mut buf);
    //         // Bypass format_args!() to avoid write_str with zero-length strs
    //         fmt::Display::fmt(self, &mut formatter)
    //             .expect("a Display implementation returned an error unexpectedly");
    //         buf
    //     }
    // }


    let s = 4.to_string();
    



}

//范型
//单态化

//编译时将范型编译成具体类型
