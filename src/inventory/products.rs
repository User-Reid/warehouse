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
