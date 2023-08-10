// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

mod item;
mod map;
mod market_place;
mod player;
mod player_actions;
mod state;
mod storage;
use player_actions::PlayerMsg;
use state::State;
use storage::{load_player, load_state, update_player, update_state};
use tezos_smart_rollup::host::RuntimeError;
use tezos_smart_rollup::kernel_entry;
use tezos_smart_rollup::prelude::Runtime;

pub fn entry<R: Runtime>(rt: &mut R) {
    rt.write_debug("Hello world");

    // Read the inbox messages
    loop {
        let input = rt.read_input();
        match input {
            Ok(Some(message)) => {
                let player_msg = PlayerMsg::try_from(message.as_ref().to_vec());
                if let Ok(player_msg) = player_msg {
                    rt.write_debug("Message is deserialized");
                    let PlayerMsg {
                        public_key: player_address,
                        action: player_action,
                    } = player_msg;

                    let other_placer = match &player_action {
                        player_actions::PlayerAction::Buy(player_address, _) => {
                            load_player(rt, player_address).ok()
                        }
                        _ => None,
                    };

                    let state: Result<State, RuntimeError> = load_state(rt, &player_address);
                    if let Ok(state) = state {
                        rt.write_debug("Calling transtion");
                        let (next_state, player) =
                            state.transition(other_placer, player_action.clone(), &player_address);
                        let _ = update_state(rt, &player_address, &next_state);
                        match player {
                            None => {}
                            Some(player) => {
                                if let player_actions::PlayerAction::Buy(address, _) =
                                    &player_action
                                {
                                    let _ = update_player(rt, address, &player);
                                }
                            }
                        }
                    }
                    //here
                }
            }
            _ => break,
        }
    }
}

kernel_entry!(entry);

#[cfg(test)]
mod tests {

    use crate::{
        item::{self, Item},
        map::{Map, TileType, MAP_HEIGHT, MAP_WIDTH},
        market_place::MarketPlace,
        player::Player,
        state,
    };

    const ITEM_X: usize = MAP_WIDTH / 2;
    const ITEM_Y: usize = MAP_HEIGHT / 2;

    fn idx_to_xy(idx: usize) -> (usize, usize) {
        let x = idx % MAP_WIDTH;
        let y = idx / MAP_HEIGHT;
        (x, y)
    }

    #[test]
    fn test_add_item() {
        let map = Map::new();
        let sword = item::Item::new_sword();

        let map = map.add_item(ITEM_X, ITEM_Y, sword);

        let is_sword = matches!(
            map.get_tile(ITEM_X, ITEM_Y),
            Some(TileType::Floor(Some(Item::Sword)))
        );

        assert!(is_sword);
    }

    #[test]
    fn test_remove_item() {
        let map = Map::new();
        let sword = item::Item::new_sword();

        let map = map.add_item(ITEM_X, ITEM_Y, sword);
        let map = map.remove_item(ITEM_X, ITEM_Y);

        let is_floor_none = matches!(map.get_tile(ITEM_X, ITEM_Y), Some(TileType::Floor(None)));

        assert!(is_floor_none);
    }

    #[test]
    fn test_add_gold() {
        let address = "Address".to_string();
        let sword_position = idx_to_xy(48);
        let state = state::State {
            map: Map::new(),
            player: Player::new(sword_position.0, sword_position.1, address),
            market_place: MarketPlace::new(),
        };
        let player = state.player.add_gold(100);
        assert_eq!(1100, player.gold)
    }

    #[test]
    fn test_remove_gold() {
        let address = "Address".to_string();
        let sword_position = idx_to_xy(48);
        let state = state::State {
            map: Map::new(),
            player: Player::new(sword_position.0, sword_position.1, address),
            market_place: MarketPlace::new(),
        };
        let player = state.player.remove_gold(100);
        assert_eq!(900, player.gold)
    }

    #[test]
    fn test_pickup_item() {
        let address = "Address".to_string();
        let sword_position = idx_to_xy(48);
        let state = state::State {
            map: Map::new(),
            player: Player::new(sword_position.0, sword_position.1, address),
            market_place: MarketPlace::new(),
        };

        let state = state.pick_up();

        let is_sword = matches!(
            state.map.get_tile(state.player.x_pos, state.player.y_pos),
            Some(TileType::Floor(None))
        );
        let state::State { player, .. } = state;
        let inventory = player.inventory;

        assert!(!inventory.is_empty());
        assert!(is_sword);
    }

    #[test]
    fn test_drop_item() {
        let address = "Address".to_string();
        let sword_position = idx_to_xy(48);
        let state = state::State {
            map: Map::new(),
            player: Player::new(sword_position.0, sword_position.1, address),
            market_place: MarketPlace::new(),
        };

        let state = state::State::pick_up(state);

        let state = state::State::drop_item(state, 0);
        let state = state::State::drop_item(state, 1);

        let is_present = matches!(
            state.map.get_tile(state.player.x_pos, state.player.y_pos),
            Some(TileType::Floor(Some(Item::Sword)))
        );

        let state::State { player, .. } = state;
        let inventory = player.inventory;

        assert!(inventory.is_empty());
        assert!(is_present);
    }
}
