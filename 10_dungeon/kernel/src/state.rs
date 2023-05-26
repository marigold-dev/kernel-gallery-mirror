use crate::{
    item::Item,
    map::Map,
    map::TileType,
    map::MAP_HEIGHT,
    map::MAP_WIDTH,
    market_place::{self, MarketPlace},
    player::{Player, MAX_ITEMS},
    player_actions::PlayerAction,
};

// Define State
#[derive(Clone, PartialEq)]
pub struct State {
    pub map: Map,
    pub player: Player,
    pub market_place: MarketPlace,
}

impl State {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(MAP_WIDTH / 2, MAP_HEIGHT / 2),
            market_place: MarketPlace::new(),
        }
    }

    pub fn pick_up(self) -> State {
        let x_pos = self.player.x_pos;
        let y_pos = self.player.y_pos;

        let tile = self.map.get_tile(x_pos, y_pos);

        match tile {
            Some(TileType::Floor(Some(item))) => {
                // player pickup the item and add to inventory
                let player = self.player.add_item(item);
                // after pickup, remove item from the map
                let map = self.map.remove_item(x_pos, y_pos);
                State {
                    player,
                    map,
                    ..self
                }
            }
            Some(TileType::Floor(None)) => self,
            _ => self,
        }
    }

    // Drop item from the inventory
    pub fn drop_item(self, item_position: usize) -> State {
        let x_pos = self.player.x_pos;
        let y_pos = self.player.y_pos;

        // check there is item in inventory or not
        let tile = self.map.get_tile(x_pos, y_pos);
        match tile {
            // we can only drop when there is nothing on the floor
            Some(TileType::Floor(None)) => {
                // remove_item of the player
                let (player, item) = self.player.remove_item(item_position);
                // get item in the inventory
                match item {
                    Some(item) => {
                        let map = self.map.add_item(x_pos, y_pos, item);
                        State {
                            player,
                            map,
                            ..self
                        }
                    }
                    None => State {
                        // the player position need to be update
                        player,
                        ..self
                    },
                }
            }
            _ => self,
        }
    }

    // Market-place: Sell (item_id, price)
    pub fn sell_item(self, current_player_address: &str, item_id: usize, price: usize) -> State {
        let item: Option<Item> = self.player.inventory.get(item_id).cloned();
        match item {
            Some(item) => {
                let mut market_place = self.market_place;
                let mut inventory = self.player.inventory;

                inventory.remove(item_id);
                let player = Player {
                    inventory,
                    ..self.player
                };
                market_place.sell_item(current_player_address, item, price);

                State {
                    player,
                    market_place,
                    ..self
                }
            }

            None => self,
        }
    }

    // Marketplace: Buy(player_address, item)
    pub fn buy_item(
        self,
        player_address: &str,
        item: Item,
        other_player: Player,
    ) -> (State, Player) {
        let price = self.market_place.get_price(player_address, item);

        // check the inventory length
        let inventory_len = self.player.inventory.len();
        if inventory_len > MAX_ITEMS {
            return (self, other_player);
        }

        let gold = &self.player.gold.clone();

        match price {
            None => (self, other_player),
            Some(price) => {
                if gold < &price {
                    return (self, other_player);
                }
                let mut market_place = self.market_place;

                market_place.buy_item(player_address, item);
                //println!("after buy item: {:?}", market_place.inner);

                // then add the item to the inventory
                let player = self.player;
                let player = player.add_item(item);
                let player = player.remove_gold(price);
                let other_player = other_player.add_gold(price);

                (
                    State {
                        player,
                        market_place,
                        ..self
                    },
                    other_player,
                )
            }
        }
    }

    fn update_player(self, player: Player) -> State {
        if self.map.can_enter_tile(player.x_pos, player.y_pos) {
            State { player, ..self }
        } else {
            self
        }
    }

    pub fn transition(
        self,
        other_player: Option<Player>,
        player_action: PlayerAction,
        current_player_address: &str,
    ) -> (State, Option<Player>) {
        match (other_player, player_action) {
            (_, PlayerAction::MoveRight) => {
                let player = self.player.clone();
                let state = self.update_player(player.move_right());
                (state, None)
            }
            (_, PlayerAction::MoveLeft) => {
                let player = self.player.clone();
                let state = self.update_player(player.move_left());
                (state, None)
            }
            (_, PlayerAction::MoveUp) => {
                let player = self.player.clone();
                let state = self.update_player(player.move_up());
                (state, None)
            }
            (_, PlayerAction::MoveDown) => {
                let player = self.player.clone();
                let state = self.update_player(player.move_down());
                (state, None)
            }
            (_, PlayerAction::PickUp) => (self.pick_up(), None),
            (_, PlayerAction::Drop(item_position)) => (self.drop_item(item_position), None),
            (_, PlayerAction::Sell(item_id, price)) => {
                let state = self.sell_item(current_player_address, item_id, price);
                (state, None)
            }
            (Some(other_player), PlayerAction::Buy(player_address, item)) => {
                let (state, player) = self.buy_item(&player_address, item, other_player);
                (state, Some(player))
            }
            _ => (self, None),
        }
    }
}
