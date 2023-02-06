#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}
//定义方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area() && self.width > other.width
    }
}

fn greater_than_100(t: i32) {
    if (t <= 100) {
        panic!("less than 100 , get {}", t);
    }
}

//单元测试
//cfg:configeration
#[cfg(test)]
mod tests {

    #[test]
    fn rename_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // #[test]
    // fn panic_works() {
    //     panic!("test fail");
    // }

    use super::*;
    #[test]

    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 20,
            height: 100,
        };

        let smaller = Rectangle {
            width: 10,
            height: 90,
        };

        assert!(larger.can_hold(&smaller));
    }

    //使用should_panic
    use super::*;
    #[test]
    #[should_panic(expected = "less than 100")]

    fn test_greater_than_100() {
        greater_than_100(10);
    }

    #[test]
    fn it_works_first() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2+2!=4"))
        }
    }

    #[test]
    fn it_works_again() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2+2!=4"))
        }
    }
}

//cargo test
//cargo test --help
//cargo test -- --help

//并行运行测试

//串行运行
//cargo test --test-threads=1

//打印成功测试的输出
//针对二进制文件的
//cargo test -- --show-output

//只跑一个测试
//例如 cargo test test_greater_than_100

//运行多个测试
// cargo test test

//忽略测试
// #[ignore]

//运行忽略测试
//cargo test --ignored
