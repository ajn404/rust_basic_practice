mod front_of_house {
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
pub use crate::front_of_house::hosting;

// pub use 重导出
fn main(){
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}