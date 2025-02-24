/// This module represents tools for management.
pub mod inventory;
/// This module represents tools for order manager.
pub mod orders;

pub use inventory::{Item, ProductCatagory, FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
pub use orders::MANAGER as ORDERS_MANAGER;
