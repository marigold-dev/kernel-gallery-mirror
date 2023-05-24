use crate::item::Item;
use std::collections::HashMap;

// Define State
#[derive(Clone, PartialEq)]
pub struct MarketPlace {
    // FA2 contract
    // key:(player_address, item_id), value
    pub inner: HashMap<(String, Item), usize>,
}

impl MarketPlace {
    pub fn new() -> Self {
        // From the beginning the Marketplace is empty
        let inner = HashMap::new();
        MarketPlace { inner }
    }

    pub fn get_price(&self, player_address: &str, item: Item) -> Option<usize> {
        self.inner.get(&(player_address.to_string(), item)).copied()
    }

    // Add buy

    pub fn buy_item(&mut self, player_address: &str, item: Item) {
        self.inner.remove(&(player_address.to_string(), item));
    }

    // Add sell

    pub fn sell_item(&mut self, current_player_address: &str, item: Item, price: usize) {
        self.inner
            .insert((current_player_address.to_string(), item), price);
    }
}
