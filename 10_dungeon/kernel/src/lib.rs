mod map;
mod player;

mod item;
mod player_actions;
mod state;
mod storage;
use player_actions::PlayerMsg;
use state::State;
use storage::{load_state, update_state};
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
                        let state: Result<State, RuntimeError> = load_state(rt, &player_address);
                        match state {
                            Ok(state) => {
                                rt.write_debug("Calling transtion");
                                let next_state = state.transition(player_action);
                                let _ = update_state(rt, &player_address, &next_state);
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
        map::{TileType, MAP_HEIGHT, MAP_WIDTH},
        state,
    };

    const ITEM_X: usize = MAP_WIDTH / 2;
    const ITEM_Y: usize = MAP_HEIGHT / 2;

    #[test]
    fn test_add_item() -> () {
        let mut state = state::State::new();
        let sword = item::Item::new_sword();
        let _add_sword = &state.map.add_item(ITEM_X, ITEM_Y, sword);
        let is_sword = match &state.map.get_tile(ITEM_X, ITEM_Y) {
            Some(TileType::Floor(Some(Item::Sword))) => true,
            _ | None => false,
        };

        assert_eq!(is_sword, false);
    }

    fn test_remove_item() -> () {
        let mut state = state::State::new();
        let sword = item::Item::new_sword();
        let _add_sword = &state.map.add_item(ITEM_X, ITEM_Y, sword);
        let mut map = &state.map.remove_item(ITEM_X, ITEM_Y);
        let is_floor_none = match map.get_tile(ITEM_X, ITEM_Y) {
            Some(TileType::Floor(None)) => true,
            _ | None => false,
        };

        assert_eq!(is_floor_none, true);
    }

    fn test_pickup_item() -> () {
        let mut state = state::State::new();
        let sword = item::Item::new_sword();
        let _add_sword = &state.map.add_item(ITEM_X, ITEM_Y, sword);

        let state = state::State::pick_up(&state);

        let is_sword = match state.map.get_tile(state.player.x_pos, state.player.y_pos) {
            Some(TileType::Floor(None)) => true,
            _ | None => false,
        };

        assert_eq!(is_sword, false);
    }

    fn test_drop_item() -> () {
        let mut state = state::State::new();
        let sword = item::Item::new_sword();
        let _add_sword = &state.map.add_item(ITEM_X, ITEM_Y, sword);

        let state = state::State::pick_up(state);

        let state = state::State::drop_item(state, 0);

        let is_sword = match state.map.get_tile(state.player.x_pos, state.player.y_pos) {
            Some(TileType::Floor(Some(Item::Sword))) => true,
            _ | None => false,
        };

        assert_eq!(is_sword, false);
    }
}
