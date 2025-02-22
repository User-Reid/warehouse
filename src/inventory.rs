pub mod products;

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

pub fn talk_to_manager() {
    println!("Hey, {}, I really dont like you...🤌", MANAGER);
}
