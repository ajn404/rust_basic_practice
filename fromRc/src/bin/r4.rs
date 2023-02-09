trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}
impl Draw for Button {
    fn draw(&self) {
        println!("这是屏幕上第{}号按钮", self.id)
    }
}

struct Select {
    id: u32,
}

impl Draw for Select {
    fn draw(&self) {
        println!("这个选择框贼难用{}", self.id)
    }
}

fn main() {
    let elems: Vec<Box<dyn Draw>> =
        vec![Box::new(Button { id: 10 }), Box::new(Select { id: 9999 })];
    for e in elems {
        e.draw();
    }
    //dyn Draw是动态分发trait类型，表示实现了Draw trait的任何类型。

}
