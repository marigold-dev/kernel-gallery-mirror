use crate::{
    map::Map, map::TileType, map::MAP_HEIGHT, map::MAP_WIDTH, player::Player,
    player_actions::PlayerAction,
};

// Define State
#[derive(Clone, PartialEq)]
pub struct State {
    pub map: Map,
    pub player: Player,
}

impl State {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(MAP_WIDTH / 2, MAP_HEIGHT / 2),
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
                State { player, map }
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
                        State { player, map }
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

    fn update_player(self, player: Player) -> State {
        if self.map.can_enter_tile(player.x_pos, player.y_pos) {
            State { player, ..self }
        } else {
            self
        }
    }

    pub fn transition(self, player_action: PlayerAction) -> State {
        match player_action {
            PlayerAction::MoveRight => {
                let player = self.player.clone();
                self.update_player(player.move_right())
            }
            PlayerAction::MoveLeft => {
                let player = self.player.clone();
                self.update_player(player.move_left())
            }
            PlayerAction::MoveUp => {
                let player = self.player.clone();
                self.update_player(player.move_up())
            }
            PlayerAction::MoveDown => {
                let player = self.player.clone();
                self.update_player(player.move_down())
            }
            PlayerAction::PickUp => self.pick_up(),
            PlayerAction::Drop(item_position) => self.drop_item(item_position),
        }
    }
}
