use crate::item::Item;

pub const MAX_ITEMS: usize = 2;

#[derive(Clone, PartialEq)]
pub struct Player {
    pub address: String,
    pub x_pos: usize,
    pub y_pos: usize,
    pub inventory: Vec<Item>,
    pub gold: usize,
}

impl Player {
    pub fn new(x_pos: usize, y_pos: usize, address: String) -> Self {
        Self {
            address,
            x_pos,
            y_pos,
            inventory: Vec::new(),
            // default 1000 gold for each player
            gold: 1000,
        }
    }

    pub fn move_up(&self) -> Player {
        usize::checked_sub(self.y_pos, 1)
            .map(|y_pos| Self {
                address: self.address.clone(),
                y_pos,
                x_pos: self.x_pos,
                inventory: self.inventory.clone(),
                gold: self.gold,
            })
            .unwrap_or(self.clone())
    }

    pub fn move_down(&self) -> Player {
        Self {
            address: self.address.clone(),
            y_pos: self.y_pos + 1,
            x_pos: self.x_pos,
            inventory: self.inventory.clone(),
            gold: self.gold,
        }
    }

    pub fn move_left(&self) -> Player {
        usize::checked_sub(self.x_pos, 1)
            .map(|x_pos| Self {
                address: self.address.clone(),
                x_pos,
                y_pos: self.y_pos,
                inventory: self.inventory.clone(),
                gold: self.gold,
            })
            .unwrap_or(self.clone())
    }

    pub fn move_right(&self) -> Player {
        Self {
            address: self.address.clone(),
            x_pos: self.x_pos + 1,
            y_pos: self.y_pos,
            inventory: self.inventory.clone(),
            gold: self.gold,
        }
    }

    // add item to inventory
    pub fn add_item(self, item: Item) -> Player {
        let inventory_len = self.inventory.len();

        if inventory_len <= MAX_ITEMS {
            let mut inventory = self.inventory;

            inventory.push(item);

            Player { inventory, ..self }
        } else {
            self
        }
    }

    // remove item from the inventory of the player
    pub fn remove_item(self, item_position: usize) -> (Player, Option<Item>) {
        let inventory_len = self.inventory.len();

        // check that item is not more than what store inside the inventory
        if item_position < inventory_len {
            let item = self.inventory.get(item_position).cloned();
            let mut inventory = self.inventory;

            inventory.remove(item_position);

            (Player { inventory, ..self }, item)
        } else {
            (self, None)
        }
    }

    pub fn remove_gold(self, amount: usize) -> Player {
        Player {
            gold: self.gold - amount,
            ..self
        }
    }

    pub fn add_gold(self, amount: usize) -> Player {
        Player {
            gold: self.gold + amount,
            ..self
        }
    }
}
