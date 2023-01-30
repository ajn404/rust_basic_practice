// Vector 获取

fn main() {
    let v = vec![1, 2, 3, 4];
    let third = &v[2];
    println!("{}", third);
    match v.get(2) {
        Some(third) => {
            println!(
                "match get third

            "
            )
        }
        None => {
            println!(" match get None")
        }
    }

    //索引越界，match会返回none ,不会panic
}
