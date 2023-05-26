use std::collections::HashMap;

use crate::item::Item;
use crate::map::{Map, TileType, MAP_HEIGHT, MAP_WIDTH};
use crate::market_place::MarketPlace;
use crate::player::{Player, MAX_ITEMS};

use crate::state::State;
use tezos_smart_rollup_host::path::{concat, OwnedPath, RefPath};
use tezos_smart_rollup_host::runtime::{Runtime, RuntimeError};

const PLAYER_PATH: RefPath = RefPath::assert_from(b"/players");
const MAP_PATH: RefPath = RefPath::assert_from(b"/state/map");
const MARKET_PLACE_PATH: RefPath = RefPath::assert_from(b"/market-place");

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

// Now we define the /players/{public_key}/x_pos
fn player_x_pos(player_address: &str) -> OwnedPath {
    // convert it to a string, and we are sure that it is a string so we can unwrap it
    // then we can call the OwnedPath::try_from
    let x_pos_path = OwnedPath::try_from("/x_pos".to_string()).unwrap();

    let player_path = player_key(player_address);
    concat(&player_path, &x_pos_path).unwrap()
}

// Now we define the /players/{public_key}/y_pos
fn player_y_pos(player_address: &str) -> OwnedPath {
    // convert it to a string, and we are sure that it is a string so we can unwrap it
    // then we can call the OwnedPath::try_from
    let y_pos_path = OwnedPath::try_from("/y_pos".to_string()).unwrap();

    let player_path = player_key(player_address);
    concat(&player_path, &y_pos_path).unwrap()
}

// Now we define the /players/{public_key}/inventory
fn player_inventory(player_address: &str) -> OwnedPath {
    // convert it to a string, and we are sure that it is a string so we can unwrap it
    // then we can call the OwnedPath::try_from
    let inventory_path = OwnedPath::try_from("/inventory".to_string()).unwrap();

    let player_path = player_key(player_address);
    concat(&player_path, &inventory_path).unwrap()
}

// Now we define the /players/{public_key}/gold
fn player_gold(player_address: &str) -> OwnedPath {
    let gold_path = OwnedPath::try_from("/gold".to_string()).unwrap();
    let player_path = player_key(player_address);
    concat(&player_path, &gold_path).unwrap()
}

// Market-place

// market-place key: /market-place/{public_key}/[01/02]/value
// sword: 01
// potion: 02
fn market_place_key(player_address: &str) -> OwnedPath {
    let player_address: Vec<u8> = format!("/{}", player_address).into();
    let player_address = OwnedPath::try_from(player_address).unwrap();
    concat(&MARKET_PLACE_PATH, &player_address).unwrap()
}

// marketplace item_01
fn market_place_sword(player_address: &str) -> OwnedPath {
    let sword_id_path = OwnedPath::try_from("/01".to_string()).unwrap();
    let market_place_path = market_place_key(player_address);
    concat(&market_place_path, &sword_id_path).unwrap()
}

// add value for item_01
fn market_place_value_sword(player_address: &str) -> OwnedPath {
    let sword_id_path = market_place_sword(player_address);
    let value_sword_path = OwnedPath::try_from("/value".to_string()).unwrap();
    concat(&sword_id_path, &value_sword_path).unwrap()
}

// marketplace item_02
fn market_place_potion(player_address: &str) -> OwnedPath {
    let potion_id_path = OwnedPath::try_from("/02".to_string()).unwrap();
    let market_place_path = market_place_key(player_address);
    concat(&market_place_path, &potion_id_path).unwrap()
}

// add value for item_02
fn market_place_value_potion(player_address: &str) -> OwnedPath {
    let potion_id_path = market_place_potion(player_address);
    let value_potion_path = OwnedPath::try_from("/value".to_string()).unwrap();
    concat(&potion_id_path, &value_potion_path).unwrap()
}

// load_player for load_state
fn load_player<R: Runtime>(rt: &mut R, player_address: &str) -> Result<Player, RuntimeError> {
    let player_path = player_key(player_address);

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

            // convert bytes -> vector
            let inventory: Vec<Item> = inventory_bytes
                .iter()
                .filter_map(|bytes| match bytes {
                    0x01 => Some(Item::Sword),
                    0x02 => Some(Item::Potion),
                    _ => None,
                })
                .collect();

            let gold = rt.store_read(
                &player_gold(player_address),
                0,
                std::mem::size_of::<usize>(),
            )?;

            // convert bytes -> usize
            let gold = usize::from_be_bytes(gold.try_into().unwrap());

            Ok(Player {
                x_pos,
                y_pos,
                inventory,
                gold,
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

fn load_market_place<R: Runtime>(rt: &mut R) -> Result<MarketPlace, RuntimeError> {
    let market_place_exists = rt.store_has(&MARKET_PLACE_PATH)?;
    println!("load market place");
    match market_place_exists {
        Some(_) => {
            // First we start from 0 to 8
            let index_size: Vec<u8> = rt.store_read(&MARKET_PLACE_PATH, 0, 8)?; // because usize is represented with 8 bytes
            println!("{:?}", index_size);
            let index_size = index_size
                .try_into()
                .map_err(|_| RuntimeError::DecodingError)?;
            let index_size = usize::from_be_bytes(index_size);
            println!("load index");

            // then we read again start from 8 to the size of the index
            let index = rt.store_read(&MARKET_PLACE_PATH, 8, index_size)?;
            let index: Vec<(String, Item)> =
                bincode::deserialize(&index).map_err(|_| RuntimeError::DecodingError)?;

            // create a new market place
            let mut inner = HashMap::new();
            println!("load inner");

            for (player_address, item) in index {
                match item {
                    Item::Sword => {
                        println!("load sword");
                        let path = &market_place_value_sword(&player_address);
                        println!("{:?}", &path);
                        let price = rt.store_read(path, 0, 8)?; // 8 because the price is an usize
                        println!("{:?}", &price);

                        let price = price.try_into().map_err(|_| RuntimeError::DecodingError)?;
                        let price = usize::from_be_bytes(price);
                        println!("sword price");
                        let _ = inner.insert((player_address, item), price);
                    }
                    Item::Potion => {
                        let path = &market_place_value_potion(&player_address);
                        let price = rt.store_read(path, 0, 8)?;
                        let price = price.try_into().map_err(|_| RuntimeError::DecodingError)?;
                        let price = usize::from_be_bytes(price);
                        // insert the new price

                        let _ = inner.insert((player_address, item), price);
                    }
                }
            }

            println!("-----------------------");
            println!("{:?}", &inner);

            Ok(MarketPlace { inner })
        }
        // if there is none then it is new hashmap using the default
        _ => Ok(MarketPlace::default()),
    }
}

// load_state will call the load_olayer and load_map
pub fn load_state<R: Runtime>(rt: &mut R, player_address: &str) -> Result<State, RuntimeError> {
    let player = load_player(rt, player_address)?;
    let map = load_map(rt)?;
    let market_place = load_market_place(rt)?;

    Ok(State {
        player,
        map,
        market_place,
    })
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

    // inventory: vec -> bytes
    let inventory: Vec<u8> = player
        .inventory
        .iter()
        .map(|item| match item {
            Item::Sword => 0x01,
            Item::Potion => 0x02,
        })
        .collect();

    let () = rt.store_write(&player_inventory(player_address), &inventory, 0)?;

    // gold: usize -> bytes
    let gold = usize::to_be_bytes(player.gold);
    let () = rt.store_write(&player_gold(&player_address), &gold, 0)?;

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

// todo update_marketplace
fn update_market_place<R: Runtime>(
    rt: &mut R,
    market_place: &MarketPlace,
) -> Result<(), RuntimeError> {
    // delete the market place
    rt.store_delete(&MARKET_PLACE_PATH)?;
    // to replace it by the updated one

    // update index
    let index: Vec<(String, Item)> = market_place.inner.keys().cloned().collect();
    let bytes = bincode::serialize(&index).map_err(|_| RuntimeError::DecodingError)?;
    let bytes_size = bytes.len().to_be_bytes();

    // 00000032[list]
    // save data with non specific size
    rt.store_write(&MARKET_PLACE_PATH, &bytes_size, 0)?;
    rt.store_write(&MARKET_PLACE_PATH, &bytes, bytes_size.len())?;

    // matching the item to write
    for ((address, item), price) in &market_place.inner {
        let price = price.to_be_bytes();
        match item {
            Item::Potion => {
                let () = rt.store_write(&market_place_value_potion(address), &price, 0)?;
            }
            Item::Sword => {
                let () = rt.store_write(&market_place_value_sword(address), &price, 0)?;
            }
        }
    }

    println!("market place saved");

    Ok(())
}

pub fn update_state<R: Runtime>(
    rt: &mut R,
    player_address: &str,
    state: &State,
) -> Result<(), RuntimeError> {
    update_player(rt, player_address, &state.player)?;
    update_map(rt, &state.map)?;
    update_market_place(rt, &state.market_place)?;
    Ok(())
}
