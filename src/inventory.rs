pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

#[derive(Debug)]
pub enum ProductCatagory {
    Ladders,
    Hammers,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub item_category: ProductCatagory,
    pub quantity: u32,
}

pub fn talk_to_manager() {
    println!("Hey, {MANAGER}, I really dont like you...🤌")
}
