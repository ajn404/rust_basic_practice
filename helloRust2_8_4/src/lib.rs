//关联类型
#[cfg(test)]
mod relevance_type {
    #[test]
    fn deep_known_trait() {
        pub trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }

        #[derive(Debug)]
        struct Couter {}
        impl Iterator for Couter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                Some(23)
            }
        }

        let mut x = Couter {};
        dbg!(x.next());
    }

    #[test]
    fn use_generics() {
        trait Container<A, B> {
            fn contains(&self, a: A, b: B) -> bool;
        }

        fn difference<A, B, C>(container: &C)
        where
            C: Container<A, B>,
        {
        }
    }

    #[test]
    fn use_relevence_type() {
        trait Container {
            type A;
            type B;
            fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
        }
        fn difference<C: Container>(container: &C) {}
    }
}

//默认范型类型
#[cfg(test)]
mod default {
    use std::process::Output;

    //使用范型参数时，可以为其指定一个默认的具体类型
    //当用户不指定RHS时，默认使用两个相同类型的值进行相加，返回一个关联类型Output
    #[test]
    fn example() {
        trait Add<RHS = Self> {
            type Output;
            fn add(self, rhs: RHS) -> Self::Output;
        }
    }

    //运算符重载：对已有的运算符重新进行定义，赋予其另一个功能，以适应不同的数据类型
    #[test]
    fn same() {
        use std::ops::Add;
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        impl Add for Point {
            type Output = Point;
            fn add(self, rhs: Self) -> Self::Output {
                Point {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }
        let x1 = Point { x: 1, y: 2 };
        let x2 = Point { x: 2, y: 2 };
        let x = x1.add(x2);
        dbg!(x);
    }

    //不同的类型相加

    #[test]
    fn different() {
        use std::ops::Add;
        #[derive(Debug)]
        struct Millimeters(u32);
        // //
        // let x = Millimeters(12);
        // dbg!(x);
        struct Meters(u32);
        impl Add<Meters> for Millimeters {
            type Output = Millimeters;
            fn add(self, other: Meters) -> Millimeters {
                Millimeters(self.0 + (other.0 * 1000))
            }
        }
    }
}

#[cfg(test)]
mod same_name {
    #[test]
    fn example_01() {
        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }
        struct Human;
        impl Pilot for Human {
            fn fly(&self) {
                eprintln!("fly fly form Pilot ! This is your catain speaking .!!")
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                eprintln!("up !!!!!!");
            }
        }

        impl Human {
            fn fly(&self) {
                eprintln!("self impl! yoh v yoh yoh yoh");
            }
        }

        let person = Human;
        person.fly();
        //调用类型Human自身的方法

        //调用特征上的方法
        //pilot
        Pilot::fly(&person);
        //wizard
        Wizard::fly(&person);
    }

    //如果方法不包含关联函数，变成了关联函数
    #[test]
    fn example_02() {
        trait Animal {
            fn baby_name() -> String;
        }
        struct Dog;
        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }

        eprintln!("{}", Dog::baby_name());
        //完全限定语法
        //as
        eprintln!("{}", <Dog as Animal>::baby_name());
    }
}

//特征定义中的特征约束
#[cfg(test)]
mod trait_bounds {
    #[test]
    fn example_01() {
        use std::fmt::Display;

        //如果你想要实现 OutlinePrint 特征，首先你需要实现 Display 特征。
        trait OutlinePrint: Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                eprintln!("{}", "*".repeat(len + 4));
                eprintln!("*{}*", " ".repeat(len + 2));
                eprintln!("* {} *", output);
                eprintln!("*{}*", " ".repeat(len + 2));
                eprintln!("{}", "*".repeat(len + 4));
            }
        }

        struct Point {
            x: i32,
            y: i32,
        }

        impl Display for Point {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{},,{}", self.x, self.y)
            }
        }

        impl OutlinePrint for Point {}

        let x = Point { x: 1, y: 2 };
        x.outline_print();
    }
}

#[cfg(test)]
mod new_type {
    #[test]
    fn example() {
        use std::fmt;

        struct Wrapper(Vec<String>);
        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[{}]", self.0.join(","))
                //Rust 提供了一个特征叫 Deref，实现该特征后，可以自动做一层类似类型转换的操作，可以将 Wrapper 变成 Vec<String> 来使用
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w={}", w);
    }
}
