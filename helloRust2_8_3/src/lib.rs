#[cfg(test)]

pub trait Draw {
    fn draw(&self) -> String;
}

#[derive(Debug)]

enum UiObejct {
    Button,
    SelectBox,
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
mod tests {

    use std::fmt::format;

    use super::*;
    #[test]
    fn example_01() {
        let obj = [UiObejct::Button, UiObejct::SelectBox];
        for ite in obj {
            draw(ite);
        }

        fn draw(ite: UiObejct) {
            eprintln!("{:?}", ite);
        }
    }

    //dyn draw
    #[test]
    fn dyn_trait_box() {
        impl Draw for u8 {
            fn draw(&self) -> String {
                format!("u8:{}", *self)
            }
        }

        impl Draw for f64 {
            fn draw(&self) -> String {
                format!("f64:{}", *self)
            }
        }

        //若 T 实现了 Draw 特征，
        //则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
        fn draw_01(x: Box<dyn Draw>) -> String {
            // 由于实现了 Deref 特征，
            // Box 智能指针会自动解引用为它所包裹的值，
            // 然后调用该值对应的类型上定义的 `draw` 方法
            x.draw()
        }

        fn draw_02(x: &dyn Draw) -> String {
            x.draw()
        }

        let x = 1.1f64;
        let y = 8u8;
        let fx = draw_01(Box::new(x));
        let fy = draw_01(Box::new(y));
        dbg!(fx);
        dbg!(fy);
        let dx = draw_02(&x);
        dbg!(dx);
    }

    #[test]
    fn dyn_components() {
        impl Draw for Button {
            fn draw(&self) -> String {
                // 绘制按钮的代码
                format!("button{}{}{}", self.width, self.height, self.label)
            }
        }

        impl Draw for SelectBox {
            fn draw(&self) -> String {
                let mut op = "".to_string();
                for str in &self.options {
                    let temp = (*str).to_string();
                    eprintln!("{}", temp);
                    op += &temp;
                }
                // 绘制SelectBox的代码
                format!("selectbox{}{}{}", self.width, self.height, op)
            }
        }

        pub struct Screen {
            pub components: Vec<Box<dyn Draw>>,
        }

        impl Screen {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }

        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 50,
                    height: 100,
                    options: vec![
                        String::from("value"),
                        String::from("value"),
                        String::from("value"),
                        String::from("value"),
                    ],
                }),
                Box::new(Button {
                    width: 20,
                    height: 100,
                    label: "button".to_string(),
                }),
            ],
        };

        screen.run();
        /*
        Screen 在 run 的时候，我们并不需要知道各个组件的具体类型是什么。
        它也不检查组件到底是 Button 还是 SelectBox 的实例，只要它实现了 Draw 特征，
        就能通过 Box::new 包装成 Box<dyn Draw> 特征对象，然后被渲染在屏幕上。
         */
    }

    //特征对象的动态分发

    #[test]
    fn dyn_dispatch() {
        //https://course.rs/basic/trait/trait-object.html#%E7%89%B9%E5%BE%81%E5%AF%B9%E8%B1%A1%E7%9A%84%E5%8A%A8%E6%80%81%E5%88%86%E5%8F%91
    }

    #[test]
    fn self_Self() {
        trait Draw {
            fn draw(&self) -> Self;
        }

        #[derive(Clone)]
        struct Button;

        impl Draw for Button {
            fn draw(&self) -> Self {
                self.clone()
            }
        }

        let btn = Button;
        let newb = btn.draw();
    }
}
