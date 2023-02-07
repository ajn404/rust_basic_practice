use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

/*
每一个闭包实例都有独属于自己的类型，
即使于两个签名一模一样的闭包，
它们的类型也是不同的，
因此你无法用一个统一的类型来标注 calculation 闭包。
*/
 
fn workout(intensity: u32, random_number: u32) {
    let mut action = Cacher::new(|intensity: u32| {
        println!("muuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25 {
        println!("今天活力满满，先做 {} 个俯卧撑!", action.value(intensity));
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            action.value(intensity)
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            action.value(intensity)
        );
    }
}

fn main() {
    // 动作次数
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 7;

    // 开始健身
    workout(intensity, random_number);
}
