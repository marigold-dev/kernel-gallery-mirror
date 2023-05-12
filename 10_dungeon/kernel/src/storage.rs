use crate::item::Item;
use crate::map::{Map, TileType, MAP_HEIGHT, MAP_WIDTH};
use crate::player::{Player, MAX_ITEMS};
use crate::state::State;
use tezos_smart_rollup_host::path::{concat, OwnedPath, RefPath};
use tezos_smart_rollup_host::runtime::{Runtime, RuntimeError};

const PLAYER_PATH: RefPath = RefPath::assert_from(b"/players");
const MAP_PATH: RefPath = RefPath::assert_from(b"/state/map");

// player key: /players/{public_key}/[x_pos|y_pos|inventory]
// Add a player key bytaking the player address, from the PLAYER_PATH
// &str: to take a reference of string
fn player_key(player_address: &str) -> OwnedPath {
    // define the suffix of player address
    let player_address: Vec<u8> = format!("/{}", player_address).into();
    // we have to use the OwnedPath to do the define of the path
    // OwnedPath and RefPath "maybe" the same in kernel?
    let player_address = OwnedPath::try_from(player_address).unwrap();

    // now we concat the prefix is b"/players/, and the suffix is the player address
    concat(&PLAYER_PATH, &player_address).unwrap()
}

// Now we define de /players/{public_key}/x_pos
fn player_x_pos(player_address: &str) -> OwnedPath {
    // convert it to a string, and we are sure that it is a string so we can unwrap it
    // then we can call the OwnedPath::try_from
    let x_pos_path = OwnedPath::try_from("/x_pos".to_string()).unwrap();

    let player_path = player_key(player_address);
    concat(&player_path, &x_pos_path).unwrap()
}

// Now we define de /players/{public_key}/y_pos
fn player_y_pos(player_address: &str) -> OwnedPath {
    // convert it to a string, and we are sure that it is a string so we can unwrap it
    // then we can call the OwnedPath::try_from
    let y_pos_path = OwnedPath::try_from("/y_pos".to_string()).unwrap();

    let player_path = player_key(player_address);
    concat(&player_path, &y_pos_path).unwrap()
}

// Now we define de /players/{public_key}/inventory
fn player_inventory(player_address: &str) -> OwnedPath {
    // convert it to a string, and we are sure that it is a string so we can unwrap it
    // then we can call the OwnedPath::try_from
    let inventory_path = OwnedPath::try_from("/inventory".to_string()).unwrap();

    let player_path = player_key(player_address);
    concat(&player_path, &inventory_path).unwrap()
}

// load_player for load_state
fn load_player<R: Runtime>(rt: &mut R, player_address: &str) -> Result<Player, RuntimeError> {
    let player_path = player_key(player_address);
    let inventory_path = player_inventory(player_address);

    let player_exists = rt.store_has(&player_path)?;

    match player_exists {
        Some(_) => {
            let x_pos = rt.store_read(
                &player_x_pos(&player_address),
                0,
                std::mem::size_of::<usize>(),
            )?;

            let y_pos = rt.store_read(
                &player_y_pos(&player_address),
                0,
                std::mem::size_of::<usize>(),
            )?;

            // convert player from usize, catch the list into an array
            let x_pos = usize::from_be_bytes(x_pos.try_into().unwrap());
            let y_pos = usize::from_be_bytes(y_pos.try_into().unwrap());

            // inventory also binid with the player_address
            let inventory_bytes =
                rt.store_read(&player_inventory(&player_address), 0, MAX_ITEMS)?;

            // convert vector of inventory to bytes
            let inventory: Vec<Item> = inventory_bytes
                .iter()
                .filter_map(|bytes| match bytes {
                    0x01 => Some(Item::Sword),
                    0x02 => Some(Item::Potion),
                    _ => None,
                })
                .collect();

            Ok(Player {
                x_pos,
                y_pos,
                inventory,
            })
        }
        _ => Ok(Player::new(MAP_WIDTH / 2, MAP_HEIGHT / 2)),
    }
}

// load_map for the load_state
fn load_map<R: Runtime>(rt: &mut R) -> Result<Map, RuntimeError> {
    // checking in the durable storage to see if there is a map or not
    let map_exists = rt.store_has(&MAP_PATH)?;
    match map_exists {
        // if not create a new one
        None => Ok(Map::new()),
        // if there is something then read it from the durable storage
        Some(_) => {
            let map_bytes = rt.store_read(&MAP_PATH, 0, MAP_WIDTH * MAP_HEIGHT)?;

            let tiles: Vec<TileType> = map_bytes
                .iter()
                .filter_map(|bytes| match bytes {
                    0x01 => Some(TileType::Floor(Some(Item::Sword))),
                    0x02 => Some(TileType::Floor(Some(Item::Potion))),
                    0x03 => Some(TileType::Floor(None)),
                    0x04 => Some(TileType::Wall),
                    _ => None,
                })
                .collect();

            // define the map with the tiles
            let map = Map { tiles };
            Ok(map)
        }
    }
}

// load_state will call the load_olayer and load_map
pub fn load_state<R: Runtime>(rt: &mut R, player_address: &str) -> Result<State, RuntimeError> {
    // first load the player
    // TODO: player_position --> player
    let player = load_player(rt, player_address)?;
    // then load the map
    let map = load_map(rt)?;
    // return the new state

    Ok(State { player, map })
}

fn update_player<R: Runtime>(
    rt: &mut R,
    player_address: &str,
    player: &Player,
) -> Result<(), RuntimeError> {
    // player position: convert vector back to bytes, and store it in the x_pos_path
    let x_pos = usize::to_be_bytes(player.x_pos);
    let () = rt.store_write(&player_x_pos(&player_address), &x_pos, 0)?;

    let y_pos = usize::to_be_bytes(player.y_pos);
    let () = rt.store_write(&player_y_pos(&player_address), &y_pos, 0)?;

    let inventory: Vec<u8> = player
        .inventory
        .iter()
        .map(|item| match item {
            Item::Sword => 0x01,
            Item::Potion => 0x02,
        })
        .collect();

    let () = rt.store_write(&player_inventory(player_address), &inventory, 0)?;

    Ok(())
}

fn update_map<R: Runtime>(rt: &mut R, map: &Map) -> Result<(), RuntimeError> {
    let tiles: Vec<u8> = map
        .tiles
        .iter()
        .map(|tile_type| match tile_type {
            TileType::Floor(Some(Item::Sword)) => 0x01,
            TileType::Floor(Some(Item::Potion)) => 0x02,
            TileType::Floor(None) => 0x03,
            TileType::Wall => 0x04,
        })
        .collect();

    let () = rt.store_write(&MAP_PATH, &tiles, 0)?;

    Ok(())
}

pub fn update_state<R: Runtime>(
    rt: &mut R,
    player_address: &str,
    state: &State,
) -> Result<(), RuntimeError> {
    update_player(rt, player_address, &state.player)?;
    update_map(rt, &state.map)?;

    Ok(())
}
