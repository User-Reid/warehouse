use fake::{Fake, Faker};

use warehouse::*;

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
