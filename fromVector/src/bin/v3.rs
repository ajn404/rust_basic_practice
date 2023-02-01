enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main(){
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.20),
        SpreadSheetCell::Text(String::from("helloworld")),
    ];

    
}