#[cfg(test)]
mod tests {
    #[test]
    fn create_vec() {
        let v: Vec<i32> = Vec::new();
        let mut x = Vec::new();
        x.push(1);

        let z: Vec<String> = Vec::with_capacity(10);
        dbg!(z);

        let mut t = vec![1, 2, 3, 4];
        t.push(100);
        dbg!(&t);
        let third = &t[2];
        eprintln!("t的第三个元素是{}", third);
        if let Some(third) = t.get(2) {
            eprintln!("第三个元素是{third}");
        }
    }
}
