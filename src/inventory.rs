pub mod products;

pub use products::{Item, ProductCatagory};

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

fn talk_to_manager() {
    println!("Hey, {}, I really dont like you...🤌. Also, what do you think of {:#?} btw?", MANAGER, ProductCatagory::Ladders);
}
