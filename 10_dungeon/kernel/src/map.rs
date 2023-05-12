use std::vec;

use crate::item::Item;

pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;

#[derive(Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor(Option<Item>),
}

#[derive(Clone, PartialEq)]
pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: usize, y: usize) -> usize {
    (y * MAP_WIDTH) + x
}

pub fn idx_to_xy(idx: usize) -> (usize, usize) {
    let x = idx % MAP_WIDTH;
    let y = idx / MAP_HEIGHT;
    (x, y)
}

impl Map {
    // Define a new map at the beginning of the game
    pub fn new() -> Self {
        let mut map = vec![
            // add sword in the beginning on the floor
            TileType::Floor(None);
            MAP_WIDTH * MAP_HEIGHT
        ];

        // TODO: write a function convert from_string(string:String) -> Map
        // place wall at the 0
        map[0] = TileType::Wall;
        // place the sword on the 48 on the map

        map[48] = TileType::Floor(Some(Item::Sword));
        map[970] = TileType::Floor(Some(Item::Potion));

        Self { tiles: map }
    }

    #[allow(dead_code)]
    pub fn get_sword(self) -> Item {
        let sword = TileType::Floor(std::option::Option::Some(Item::Sword));
        match sword {
            TileType::Floor(Some(sword)) => sword,
            TileType::Floor(None) => todo!(),
            TileType::Wall => todo!(),
        }
    }

    #[allow(dead_code)]
    pub fn get_potion() -> Item {
        let potion = TileType::Floor(std::option::Option::Some(Item::Potion));
        match potion {
            TileType::Floor(Some(potion)) => potion,
            TileType::Floor(None) => todo!(),
            TileType::Wall => todo!(),
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<TileType> {
        // cloned: inner value of an option and not an option itself
        // using [get] to avoid of [out_of_bound] in the [map_idx] function
        self.tiles.get(map_idx(x, y)).cloned()
    }

    // player can walk on floor but not through walls, the floor can be anything
    pub fn can_enter_tile(&self, x: usize, y: usize) -> bool {
        match self.get_tile(x, y) {
            Some(TileType::Floor(_)) => true,
            _ => false,
        }
    }

    // remove item from the map
    pub fn remove_item(self, x_pos: usize, y_pos: usize) -> Self {
        match self.get_tile(x_pos, y_pos) {
            // if there is something on the floor then return none
            Some(TileType::Floor(Some(_))) => {
                let mut tiles = self.tiles;
                tiles[map_idx(x_pos, y_pos)] = TileType::Floor(None);
                Self { tiles }
            }
            _ => self,
        }
    }

    // add item after drop into the map
    pub fn add_item(self, x_pos: usize, y_pos: usize, item: Item) -> Self {
        match self.get_tile(x_pos, y_pos) {
            Some(TileType::Floor(None)) => {
                let mut tiles = self.tiles;
                tiles[map_idx(x_pos, y_pos)] = TileType::Floor(Some(item));
                Self { tiles }
            }
            _ => self,
        }
    }
}
