//! Should these systems be put into Ethnopians?
mod agent;
pub use agent::agent_system;
mod cache_inventory;
pub use cache_inventory::{CacheInventory, cache_inventory_system};
mod drop;
pub use drop::drop_system;
mod find_in_inventory;
pub use find_in_inventory::{
    FindInInventoryRequest, FindInInventoryResult, find_in_inventory_system,
};
mod salt;
pub use salt::{Salt, salt_system};
