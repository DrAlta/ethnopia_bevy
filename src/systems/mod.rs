//! Should these systems be put into Ethnopians?
pub mod actions;
mod agent;
pub use agent::agent_system;
mod cache_inventory;
pub use cache_inventory::{CacheInventory, cache_inventory_system};
pub mod change_request;
mod find_in_inventory;
pub use find_in_inventory::{
    FindInInventoryRequest, FindInInventoryResult, find_in_inventory_system,
};
//mod inventory;
pub mod movement;
pub mod posible_actions;
mod salt;
pub use salt::{Salt, salt_system};
