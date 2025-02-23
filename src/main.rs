mod inventory;
mod orders;

use std::{
    fmt,
    io::{stdin, stdout},
};

use fake::{Fake, Faker};

use inventory::{Item, ProductCatagory, FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
use orders::MANAGER as ORDERS_MANAGER;

fn main() {
    println!(
        "Our managers are {}, and {}. We also have {}sqf floor space",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );

    let fake_item: Item = Faker.fake();

    println!("{:#?}", fake_item);

    let random_category: ProductCatagory = Faker.fake();
    println!("{:#?}", random_category)
}
