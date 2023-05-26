import logo from "./logo.svg";
import "./App.css";
import { useEffect, useState } from "react";
import { InMemorySigner } from "@taquito/signer";
import { move } from "./action.js";
import { BOB_SECRET, ALICE_SECRET } from "./player_adds.js";

/**
 * Split a string into n slices
 */
const splitNChars = (txt) => {
  let result = [];
  for (let i = 0; i < txt.length; i += 2) {
    // 2 is the length of the bytes
    result.push(txt.substr(i, 2));
  }
  return result;
};

const App = () => {
  // players
  // Only contains the position of the different players
  const [players, setPlayers] = useState([]);

  // signer
  const [signer, setSigner] = useState(undefined);

  const setSecret = (secret) => {
    InMemorySigner.fromSecretKey(secret).then(setSigner);
  };

  // marketplace
  const [marketplace, setMarketplace] = useState([]);

  // At the start we use bob
  useEffect(() => {
    setSecret(BOB_SECRET);
  }, []);

  // Player actions
  const moveUp = move("01", signer);
  const moveDown = move("02", signer);
  const moveLeft = move("03", signer);
  const moveRight = move("04", signer);
  const pickUp = move("05", signer);
  // drop needs to know the position of the item in the inventory
  // to be able to drop them
  const drop = (itemPosition) => move(`060${itemPosition}`, signer);
  const sell = (itemPosition) => move(`070${itemPosition}`, signer);
  const buy = (address, itemId) => move(`080${itemId}${address}`, signer);

  // handle keyboards
  const onKeyDown = (e) => {
    // 37 = left, 38 = up, 39 = right, 40 = down
    // console.log("hello world");
    // console.log(e.key);
    // console.log(e.keyCode);
    if (e.keyCode === 38) {
      // need to return an () after calling the function
      move("01", signer)();
    } else if (e.keyCode === 40) {
      move("02", signer)();
    } else if (e.keyCode === 37) {
      move("03", signer)();
    } else if (e.keyCode === 39) {
      move("04", signer)();
    }
    // y = 89 for pickup
    else if (e.keyCode === 89) {
      move("05", signer)();
    }
  };

  // 32 * 32
  // Define a player with player position and
  // Add the inventory of the player
  // Only contain the inventory of the player
  const [player, updatePlayer] = useState({ x: 0, y: 0, inventory: [] });

  // Define a map
  // place item on map, tiles are: 01, 02, 03, 04 is a binary update_state
  // correspond with the one define in rust
  // 0x01 => Some(TileType::Floor(Some(Sword))),
  // 0x02 => Some(TileType::Floor(Some(Potion))),
  // 0x03 => Some(TileType::Floor(None)),
  // 0x04 => Some(TileType::Wall),
  const [map, updateMap] = useState({ tiles: ["01", "02", "03", "04"] });

  useEffect(() => {
    if (!signer) {
      return () => {};
    }

    // starting an interval
    const interval = setInterval(async () => {
      // to retrieve all the different players
      // to display players by using the subkeys in sequencer
      // to get all the players in the list
      const playersRes = await fetch(
        "http://127.0.0.1:8080/state/subkeys?path=/players"
      );

      // update the position of each player
      // .json to parse a text in a json
      const players = await playersRes.json();

      // go through the list of players and get their publicKeyHash
      // reduce is like fold function in OCaml
      const updatedPlayers = await players.reduce(
        async (players, playerAddress) => {
          // Player
          // Fetching the player x position, res1 is the http answer
          const res1 = await fetch(
            `http://127.0.0.1:8080/state/value?path=/players/${playerAddress}/x_pos`
          );
          // Getting the response as text
          const x_pos_bytes = await res1.text();
          // Converting the text as number
          const x_pos = Number.parseInt(x_pos_bytes, 16);

          const res2 = await fetch(
            `http://127.0.0.1:8080/state/value?path=/players/${playerAddress}/y_pos`
          );
          const y_pos_bytes = await res2.text();
          const y_pos = Number.parseInt(y_pos_bytes, 16);

          // update players by adding the end of the list the x, y position
          const acc = await players;
          return Promise.resolve([
            ...acc,
            { y: y_pos, x: x_pos, address: playerAddress },
          ]);
        },
        Promise.resolve([])
      );
      setPlayers(updatedPlayers);

      const player_address = await signer.publicKeyHash();

      // fetching the inventory of the player
      const res4 = await fetch(
        `http://127.0.0.1:8080/state/value?path=/players/${player_address}/inventory`
      );
      // inventory is a string of 2 bytes: 2 splots
      const inventory_bytes = await res4.text();
      // split the inventory to string
      const inventory = splitNChars(inventory_bytes);

      // fetching the gold of the player

      const res5 = await fetch(
        `http://127.0.0.1:8080/state/value?path=/players/${player_address}/gold`
      );

      const gold_bytes = await res5.text();
      const gold = Number.parseInt(gold_bytes, 16);

      updatePlayer({
        x: 0,
        y: 0,
        inventory,
        gold,
      });

      // Map -> Item
      const res3 = await fetch(
        "http://127.0.0.1:8080/state/value?path=/state/map"
      );

      const map_bytes = await res3.text();
      // need to split the array to a list of string
      const map = splitNChars(map_bytes);

      // we can put new value to the map
      updateMap(map);

      // fetch the market place
      console.log("fetch the market place");
      const marketplace_res = await fetch(
        "http://127.0.0.1:8080/state/subkeys?path=/market-place"
      );
      const sellers = await marketplace_res.json();

      // if there is no market place, create one
      let market_place = [];
      for (let index = 0; index < sellers.length; index++) {
        // get the address from the seller index
        const address = sellers[index];

        const itemsRes = await fetch(
          `http://127.0.0.1:8080/state/subkeys?path=/market-place/${address}`
        );
        const items = await itemsRes.json();

        // going to the items
        for (let indexItem = 0; indexItem < items.length; indexItem++) {
          const itemId = items[indexItem];

          // getting the price
          const priceRes = await fetch(
            `http://127.0.0.1:8080/state/value?path=/market-place/${address}/${itemId}/value`
          );
          const price_bytes = await priceRes.text();
          const price = Number.parseInt(price_bytes, 16);

          // push the sell into market place
          market_place.push({
            address,
            item: itemId,
            price,
          });
        }
      }
      // update the market place
      setMarketplace(market_place);
    }, 500); // The interval duration is 500ms

    return () => {
      // When the component umount, or refreshed we remove the interval
      clearInterval(interval);
    };
  }, [signer]);

  // App interface, adding the keyboard handle at App
  return (
    <div className="App" onKeyDown={onKeyDown} tabIndex="0">
      <header className="App-header">
        <div className="title">Dungeon sequencer</div>
        <div id="map">
          {Array(32)
            .fill(0)
            .map((_, map_y) => {
              return (
                // draw the map vertical
                <div key={map_y} className="line">
                  {Array(32)
                    .fill(0)
                    // draw the map horizontal
                    .map((_, map_x) => {
                      // define map_idx as in the kernel
                      let idx = map_y * 32 + map_x;

                      // place the player position on the map
                      for (let index = 0; index < players.length; index++) {
                        const player = players[index];

                        // TODO - move to the kernel later
                        // to generate the color for the player base on their address
                        const color =
                          "#" +
                          Buffer.from(player.address.substring(3, 7)) // "tz1[3-7].."
                            .toString("hex")
                            .substring(0, 6);

                        if (map_y === player.y && map_x === player.x)
                          return (
                            <div
                              key={`${index}-${map_x},${map_x}`}
                              className="cell player"
                              tabIndex={0}
                              style={{ backgroundColor: color }}
                            ></div>
                          );
                      }

                      // Place items on floor
                      // define the tile is the map_idx
                      let tile = map[idx];
                      // matching the tile and print out the items
                      switch (tile) {
                        // checking each case for the items on the map
                        case "01":
                          return (
                            <div
                              key={`${map_x},${map_x}`}
                              className="cell sword"
                              tabIndex={0}
                            ></div>
                          );
                        // potion
                        case "02":
                          return (
                            <div
                              key={`${map_x},${map_x}`}
                              className="cell potion"
                              tabIndex={0}
                            ></div>
                          );
                        // if it is none then it is just a normal cell
                        case "03":
                          return (
                            <div
                              key={`${map_x},${map_x}`}
                              className="cell"
                              tabIndex={0}
                            ></div>
                          );
                        case "04":
                          return (
                            <div
                              key={`${map_x},${map_x}`}
                              className="cell wall"
                              tabIndex={0}
                            ></div>
                          );

                        // if other cases return the white cell for floor
                        default:
                          return (
                            <div
                              key={`${map_x},${map_x}`}
                              className="cell"
                              tabIndex={0}
                            ></div>
                          );
                      }
                    })}
                </div>
              );
            })}
        </div>

        {
          // Add buttons for player actions
        }
        <div className="buttons">
          {
            // Add buttons to switch between players
          }
          <div className="buttons-players">
            <button onClick={() => setSecret(ALICE_SECRET)}>ALICE</button>
            <button onClick={() => setSecret(BOB_SECRET)}>BOB</button>
          </div>
          <button onClick={moveLeft}>left</button>

          <div className="up-down">
            <button onClick={moveUp}>up</button>
            <button onClick={moveDown}>down</button>
          </div>
          <button onClick={moveRight}>right</button>
          <button onClick={pickUp}>pick up (y)</button>
        </div>
        {
          // Display inventory belows the buttons
        }
        <div>
          <div>Inventory:</div>
          {player && player.gold && <div>Gold: {player.gold}</div>}
          {player.inventory.map((item, i) => {
            // matching the items as before for display
            switch (item) {
              case "01":
                return (
                  <div className="item">
                    {
                      // print the item cell
                    }
                    <div
                      // i : index is unit of the inventory
                      key={i}
                      className="cell sword"
                      tabIndex={0}
                    />
                    {
                      // Print the name of item kind on the right of item cell
                    }
                    <div className="item-name">Sword</div>
                    {
                      // Add the button next ot the item to drop them, only with the
                      // click
                    }
                    <button onClick={drop(i)}>drop</button>
                    <button onClick={sell(i)}>sell</button>
                  </div>
                );
              case "02":
                return (
                  <div className="item">
                    <div
                      // i : index is unit of the inventory
                      key={i}
                      className="cell potion"
                      tabIndex={0}
                    />
                    <div className="item-name">Potion</div>
                    <button onClick={drop(i)}>drop</button>
                    <button onClick={sell(i)}>sell</button>
                  </div>
                );
              default:
                return null;
            }
          })}
        </div>
        {
          // Display market place
        }
        <div style={{ marginTop: "24px" }}>
          <div>Marketplace:</div>
          {marketplace.map((item_to_sell, i) => {
            const { address, price, item } = item_to_sell;
            const item_name = item === "01" ? "sword" : "potion";

            return (
              <div>
                {address} - {item_name} - {price}{" "}
                <button onClick={buy(address, Number.parseInt(item))}>
                  buy
                </button>
              </div>
            );
          })}
        </div>
      </header>
    </div>
  );
};

export default App;
