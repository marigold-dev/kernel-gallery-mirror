use crate::item::Item;

// Define a player message with the publicKey bind to it
pub struct PlayerMsg {
    pub public_key: String,
    pub action: PlayerAction,
}

#[derive(Clone)]
pub enum PlayerAction {
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,
    PickUp,
    // item can be chosen to drop from the inventory (nth)
    Drop(usize),
    // Sell(ItemId, Price)
    Sell(usize, usize),
    // Buy(PlayerAddress, ItemId)
    Buy(String, Item),
}

// convert bytes -> playerAction need to have the implement of tryFrom
impl TryFrom<Vec<u8>> for PlayerMsg {
    type Error = ();

    fn try_from(data: Vec<u8>) -> Result<PlayerMsg, ()> {
        // 01{publicKey}-{data}: 01 come from the kernel external message
        if !data.starts_with(&[0x01]) {
            println!("not starting by 01");
            return Err(());
        }

        // 01{publicKey}-{data}
        // Remaining data, we will skip the first bytes of the message
        // and then convert it back to the vector. (01)
        let data = data.iter().skip(1).copied().collect::<Vec<u8>>();
        // {publicKey}-{data}: now we need to split this pair
        // 2d = '-'
        // we need to split in a function byte
        let mut data = data.split(|byte| byte == &0x2D);
        let public_key = data.next(); // pop the first elelement {publicKey}
        let data = data.next(); // pop the next element out {data}

        // the pop will return an optional type for both {publicKey} and {data}
        match (public_key, data) {
            (Some(public_key), Some(data)) => {
                println!("public key and data defined");
                println!("{:?}", data);
                let action = match data {
                    // First element or an array: 0x00: internal, 0x01: external
                    // second element define the bytes of player action
                    // move up
                    // the javascript of the action 0x01 --> 0x48
                    [48, 49] => Ok(PlayerAction::MoveUp),
                    // move down
                    [48, 50] => Ok(PlayerAction::MoveDown),
                    // move left
                    [48, 51] => Ok(PlayerAction::MoveLeft),
                    // move right
                    [48, 52] => Ok(PlayerAction::MoveRight),
                    // pickup
                    [48, 53] => Ok(PlayerAction::PickUp),
                    // drop with 3 bytes
                    [48, 54, 48, 48] => Ok(PlayerAction::Drop(0)),
                    [48, 54, 48, 49] => Ok(PlayerAction::Drop(1)),
                    // sell
                    [48, 55, 48, 48] => Ok(PlayerAction::Sell(0, 100)),
                    [48, 55, 48, 49] => Ok(PlayerAction::Sell(1, 100)),
                    // buy sword
                    [48, 56, 48, 49, address @ ..] => {
                        let address: &str = &String::from_utf8(address.to_vec()).map_err(|_| ())?;
                        let address: String = address.to_string();
                        println!("address of the seller: {}", address);
                        Ok(PlayerAction::Buy(address, Item::Sword))
                    }
                    // buy potion
                    [48, 56, 48, 50, address @ ..] => {
                        let address: &str = &String::from_utf8(address.to_vec()).map_err(|_| ())?;
                        let address: String = address.to_string();
                        println!("address of the seller: {}", address);
                        Ok(PlayerAction::Buy(address, Item::Potion))
                    }
                    _ => Err(()),
                }?;

                // public key is bytes we need to convert it to string
                let public_key: &str = &String::from_utf8(public_key.to_vec()).map_err(|_| ())?;
                // we have the string and in rust we have to convert it again in a string
                let public_key: String = public_key.to_string();

                // Now we can have the player message
                Ok(PlayerMsg { public_key, action })
            }
            (None, _) => {
                println!("public key is none");
                Err(())
            }
            (_, None) => {
                println!("public key is none");
                Err(())
            }
        }
    }
}
