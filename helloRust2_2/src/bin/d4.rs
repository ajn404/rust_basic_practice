use std::{fmt::Debug, sync::Arc};

fn main(){
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    report(origin)
}

fn report<T: Debug>(item:T){
    print!("{:?}",item);
}