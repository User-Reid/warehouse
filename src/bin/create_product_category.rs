use fake::{Fake, Faker};

use warehouse::*;

/// Create a fictional product category.
fn main() {
    let random_category: ProductCatagory = Faker.fake();
    println!("{:#?}", random_category)
}
