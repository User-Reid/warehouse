use fake::Dummy;

/// A category of product our business sells.
#[derive(Debug, Dummy)]
pub enum ProductCatagory {
    Ladders,
    Hammers,
}

/// A concrete item in stock within our warehouse.
#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub item_category: ProductCatagory,
    pub quantity: u32,
}

impl Item {
    /// Create a new item. :)
    pub fn new(name: String, item_category: ProductCatagory, quantity: u32) -> Self {
        super::talk_to_manager();
        Self {
            name,
            item_category,
            quantity,
        }
    }
}
