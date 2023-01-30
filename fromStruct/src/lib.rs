//模块：组织代码和定义私有边界

// mod front_of_house{
//     mod hosting{
//         fn add_to_waitlist(){}
//     }
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//super
fn server_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::server_order();
        crate::server_order();
    }
    fn cook_order() {}
    //pub struct
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("apple"),
            }
        }
    }
}

pub fn eat_at_restarant() {
    //绝对和相对路径
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();


    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("Wheat");
    print!("i'd like {} toast please",meal.toast);
    //⬇️field `seasonal_fruit` of `Breakfast` is private
    // meal.seasonal_fruit = String::from("blueberries");
}
