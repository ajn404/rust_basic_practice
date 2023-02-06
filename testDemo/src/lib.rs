pub fn add_2() -> i32 {
    2 + 2
}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works_again() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2+2!=4"))
        }
    }
}
