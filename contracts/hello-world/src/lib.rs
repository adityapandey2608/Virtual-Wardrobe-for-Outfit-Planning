#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, symbol_short};

// Represents a clothing item
#[contracttype]
#[derive(Clone)]
pub struct ClothingItem {
    pub item_id: u64,
    pub name: String,
    pub category: String, // e.g., "Top", "Bottom", "Shoes", etc.
}

// Key to store clothing items
#[contracttype]
pub enum WardrobeBook {
    Item(u64),
}

// Counter for unique item IDs
const ITEM_COUNT: Symbol = symbol_short!("ITEM_CNT");

#[contract]
pub struct VirtualWardrobeContract;

#[contractimpl]
impl VirtualWardrobeContract {
    // Function to add a clothing item
    pub fn add_item(env: Env, name: String, category: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&ITEM_COUNT).unwrap_or(0);
        count += 1;

        let item = ClothingItem {
            item_id: count,
            name,
            category,
        };

        env.storage().instance().set(&WardrobeBook::Item(count), &item);
        env.storage().instance().set(&ITEM_COUNT, &count);
        count
    }

    // View a clothing item by ID
    pub fn view_item(env: Env, item_id: u64) -> ClothingItem {
        env.storage().instance().get(&WardrobeBook::Item(item_id)).unwrap_or(ClothingItem {
            item_id: 0,
            name: String::from_str(&env, "Not Found"),
            category: String::from_str(&env, "None"),
        })
    }

    // Count total items
    pub fn total_items(env: Env) -> u64 {
        env.storage().instance().get(&ITEM_COUNT).unwrap_or(0)
    }
}
