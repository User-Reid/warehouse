const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

#[derive(Debug)]
enum ProductCatagory {
    Ladders,
    Hammers,
}

#[derive(Debug)]
struct Item {
    name: String,
    item_category: ProductCatagory,
    quantity: u32,
}

fn talk_to_manager() {
    println!("Hey, {MANAGER}, I really dont like you...🤌")
}
