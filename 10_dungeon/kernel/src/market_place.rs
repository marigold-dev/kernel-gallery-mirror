use crate::item::Item;
use std::collections::HashMap;

// Define State
#[derive(Clone, PartialEq)]
pub struct MarketPlace {
    // FA2 contract
    // key:(player_address, item_id), value
    pub inner: HashMap<(String, Item), usize>,
}

impl Default for MarketPlace {
    fn default() -> Self {
        // From the beginning the Marketplace is empty
        let inner = HashMap::new();
        MarketPlace { inner }
    }
}

impl MarketPlace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_price(&self, player_address: &str, item: Item) -> Option<usize> {
        self.inner.get(&(player_address.to_string(), item)).copied()
    }

    // Add buy

    pub fn buy_item(&mut self, player_address: &str, item: Item) {
        self.inner.remove(&(player_address.to_string(), item));
        println!("buy item: {:?}", self.inner);
    }

    // Add sell

    pub fn sell_item(&mut self, current_player_address: &str, item: Item, price: usize) {
        self.inner
            .insert((current_player_address.to_string(), item), price);
    }
}
