use std::io;


fn main() {
    let arr = ["today","is","a","good","day"];
    eprintln!("input an index");
    for _ in 0..5{
        let mut index = String::new();
        io::stdin().read_line(&mut index).expect("failed to read line");
        let index:usize = index.trim().parse().expect("Index entered was not a number");
        eprintln!("value is {}",arr[index]);
    }

}
