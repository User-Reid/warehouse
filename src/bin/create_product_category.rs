use fake::{Fake, Faker};

use warehouse::*;

fn main() {
    let random_category: ProductCatagory = Faker.fake();
    println!("{:#?}", random_category)
}
