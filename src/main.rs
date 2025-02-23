mod inventory;
mod orders;

use inventory::products::{Item, ProductCatagory};
use inventory::FLOOR_SPACE;

fn main() {
    println!(
        "Our managers are {}, and {}. We also have {}sqf floor space",
        inventory::MANAGER,
        orders::MANAGER,
        FLOOR_SPACE
    );

    let favorite_catagory = ProductCatagory::Hammers;
    println!("My favorite category of item is {favorite_catagory:?}");

    let tall_ladder = Item::new("Taco-town".to_string(), ProductCatagory::Hammers, 120);
    println!("{:#?}", tall_ladder)
}
