//函数范型
// fn largetest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
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

    let p1 = PPoint{x:20,y:"dasdas"};
    let p2 = PPoint{x:"wolao",y:12};
    let p3  = p1.mixup(p2);


    print!("p3 x={:?}\ny={:?}\n",p3.x,p3.y);

    //x,y的类型相同
}

//范型
//单态化

//编译时将范型编译成具体类型
