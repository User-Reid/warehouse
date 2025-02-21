mod inventory;
mod orders;

fn main() {
    println!(
        "Our managers are {}, and {}. We also have {}sqf floor space",
        inventory::MANAGER,
        orders::MANAGER,
        inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager();

    let favorite_catagory = inventory::products::ProductCatagory::Hammers;
    println!("My favorite category of item is {favorite_catagory:?}");

    let tall_ladder = inventory::products::Item {
        name: "Tacos".to_string(),
        item_category: favorite_catagory,
        quantity: 54,
    };

    println!("{:#?}", tall_ladder)
}
