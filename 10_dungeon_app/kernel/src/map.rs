// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

use crate::item::Item;
use std::vec;

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

impl Map {
    // Define a new map at the beginning of the game
    pub fn new() -> Self {
        let mut map = vec![
            // add sword in the beginning on the floor
            TileType::Floor(None);
            MAP_WIDTH * MAP_HEIGHT
        ];

        // Map
        for x in 0..10 {
            map[map_idx(x, 0)] = TileType::Wall;
            map[map_idx(5, 5)] = TileType::Wall;
        }

        for y in 0..10 {
            map[map_idx(0, y)] = TileType::Wall;
            map[map_idx(9, y)] = TileType::Wall;
        }

        // place sword
        map[48] = TileType::Floor(Some(Item::Sword));
        map[970] = TileType::Floor(Some(Item::Potion));

        Self { tiles: map }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<TileType> {
        // cloned: inner value of an option and not an option itself
        // using [get] to avoid of [out_of_bound] in the [map_idx] function
        self.tiles.get(map_idx(x, y)).cloned()
    }

    // player can walk on floor but not through walls, the floor can be anything
    pub fn can_enter_tile(&self, x: usize, y: usize) -> bool {
        matches!(self.get_tile(x, y), Some(TileType::Floor(_)))
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
