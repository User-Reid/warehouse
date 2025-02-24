use warehouse::*;

fn main() {
    println!(
        "Our managers are {:#?} and {:#?}. We also have {:#?}ft square space 😀",
        warehouse::INVENTORY_MANAGER,
        warehouse::ORDERS_MANAGER,
        warehouse::FLOOR_SPACE
    );
}
