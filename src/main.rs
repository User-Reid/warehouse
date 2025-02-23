mod inventory;
mod orders;

use inventory::products::{Item, ProductCatagory};
use inventory::{MANAGER as INVENTORY_MANAGER, FLOOR_SPACE};
use orders::MANAGER as ORDERS_MANAGER;

fn main() {
    println!(
        "Our managers are {}, and {}. We also have {}sqf floor space",
        INVENTORY_MANAGER,
        ORDERS_MANAGER,
        FLOOR_SPACE
    );

    let favorite_catagory = ProductCatagory::Hammers;
    println!("My favorite category of item is {favorite_catagory:?}");

    let tall_ladder = Item::new("Taco-town".to_string(), ProductCatagory::Hammers, 120);
    println!("{:#?}", tall_ladder)
}