mod map;
mod player;

mod item;
mod market_place;
mod player_actions;
mod state;
mod storage;
use market_place::MarketPlace;
use player::Player;
use player_actions::PlayerMsg;
use state::State;
use storage::{load_player, load_state, update_player, update_state};
use tezos_smart_rollup_entrypoint::kernel_entry;
use tezos_smart_rollup_host::runtime::{Runtime, RuntimeError};

// Entry
pub fn entry<R: Runtime>(rt: &mut R) {
    rt.write_debug("Hello world");

    // Read the inbox messages
    loop {
        let input = rt.read_input();
        match input {
            Ok(Some(message)) => {
                let player_msg = PlayerMsg::try_from(message.as_ref().to_vec());
                match player_msg {
                    Ok(player_msg) => {
                        rt.write_debug("Message is deserialized");
                        let PlayerMsg {
                            public_key: player_address,
                            action: player_action,
                        } = player_msg;

                        let other_placer: Option<Player> = match &player_action {
                            player_actions::PlayerAction::Buy(player_address, _) => {
                                load_player(rt, player_address).ok()
                            }
                            _ => None,
                        };

                        let state: Result<State, RuntimeError> = load_state(rt, &player_address);
                        match state {
                            Ok(state) => {
                                rt.write_debug("Calling transtion");
                                let (next_state, player) = state.transition(
                                    other_placer,
                                    player_action.clone(),
                                    &player_address,
                                );
                                let _ = update_state(rt, &player_address, &next_state);
                                match player {
                                    None => {}
                                    Some(player) => match &player_action {
                                        player_actions::PlayerAction::Buy(address, _) => {
                                            let _ = update_player(rt, &address, &player);
                                        }
                                        _ => {}
                                    },
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
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
    fn test_add_item() -> () {
        let map = Map::new();
        let sword = item::Item::new_sword();

        let map = map.add_item(ITEM_X, ITEM_Y, sword);

        let is_sword = match map.get_tile(ITEM_X, ITEM_Y) {
            Some(TileType::Floor(Some(Item::Sword))) => true,
            _ => false,
        };

        assert_eq!(is_sword, true);
    }

    #[test]
    fn test_remove_item() -> () {
        let map = Map::new();
        let sword = item::Item::new_sword();

        let map = map.add_item(ITEM_X, ITEM_Y, sword);
        let map = map.remove_item(ITEM_X, ITEM_Y);

        let is_floor_none = match map.get_tile(ITEM_X, ITEM_Y) {
            Some(TileType::Floor(None)) => true,
            _ => false,
        };

        assert_eq!(is_floor_none, true);
    }

    #[test]
    fn test_pickup_item() -> () {
        let sword_position = idx_to_xy(48);
        let state = state::State {
            map: Map::new(),
            player: Player::new(sword_position.0, sword_position.1),
            //TODO
            market_place: MarketPlace::new(),
        };

        let state = state.pick_up();

        let is_sword = match state.map.get_tile(state.player.x_pos, state.player.y_pos) {
            Some(TileType::Floor(None)) => true,
            _ => false,
        };

        let state::State { player, .. } = state;
        let inventory = player.inventory;

        assert!(!inventory.is_empty());
        assert_eq!(is_sword, true);
    }

    #[test]
    fn test_drop_item() -> () {
        let sword_position = idx_to_xy(48);
        let state = state::State {
            map: Map::new(),
            player: Player::new(sword_position.0, sword_position.1),
            //TODO
            market_place: MarketPlace::new(),
        };

        let state = state::State::pick_up(state);

        let state = state::State::drop_item(state, 0);
        let state = state::State::drop_item(state, 1);

        let is_present = match state.map.get_tile(state.player.x_pos, state.player.y_pos) {
            Some(TileType::Floor(Some(Item::Sword))) => true,
            _ => false,
        };

        let state::State { player, .. } = state;
        let inventory = player.inventory;

        assert!(inventory.is_empty());
        assert_eq!(is_present, true);
    }
}
