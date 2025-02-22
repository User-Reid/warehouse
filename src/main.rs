mod inventory;
mod orders;

use inventory::products::{self, ProductCatagory};
use inventory::{talk_to_manager, FLOOR_SPACE};

fn main() {
    println!(
        "Our managers are {}, and {}. We also have {}sqf floor space",
        inventory::MANAGER,
        orders::MANAGER,
        FLOOR_SPACE
    );

    talk_to_manager();

    let favorite_catagory = ProductCatagory::Hammers;
    println!("My favorite category of item is {favorite_catagory:?}");

    let tall_ladder = products::Item {
        name: "Tacos".to_string(),
        item_category: favorite_catagory,
        quantity: 54,
    };

    println!("{:#?}", tall_ladder)
}
