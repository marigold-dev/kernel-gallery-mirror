import logo from "./logo.svg";
import "./App.css";
import React, { Component, useEffect, useState } from "react";
import { InMemorySigner } from "@taquito/signer";
import { move } from "./action.js";
import Map from "./components/map";
import Inventory from "./components/inventory";
import MarketPlace from "./components/market_place";
import PlayerAccounts from "./components/player_accounts";
import { BOB_SECRET } from "./components/player_adds";

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
      const players_res = await fetch(
        "http://127.0.0.1:8080/state/subkeys?path=/players"
      );

      // update the position of each player
      // .json to parse a text in a json
      const players = await players_res.json();

      // go through the list of players and get their publicKeyHash
      // reduce is like fold function in OCaml
      const updatedPlayers = await players.reduce(
        async (players, playerAddress) => {
          // Player
          // Fetching the player x position, res1 is the http answer
          const x_pos_res = await fetch(
            `http://127.0.0.1:8080/state/value?path=/players/${playerAddress}/x_pos`
          );
          // Getting the response as text
          const x_pos_bytes = await x_pos_res.text();
          // Converting the text as number
          const x_pos = Number.parseInt(x_pos_bytes, 16);

          const y_pos_res = await fetch(
            `http://127.0.0.1:8080/state/value?path=/players/${playerAddress}/y_pos`
          );
          const y_pos_bytes = await y_pos_res.text();
          const y_pos = Number.parseInt(y_pos_bytes, 16);

          // update players by adding the end of the list the x, y position
          const acc = await players;
          return Promise.resolve([
            ...acc,
            { y: y_pos, x: x_pos, address: playerAddress }
          ]);
        },
        Promise.resolve([])
      );
      setPlayers(updatedPlayers);

      const player_address = await signer.publicKeyHash();

      // fetching the inventory of the player
      const inventory_res = await fetch(
        `http://127.0.0.1:8080/state/value?path=/players/${player_address}/inventory`
      );
      // inventory is a string of 2 bytes: 2 splots
      const inventory_bytes = await inventory_res.text();
      // split the inventory to string
      const inventory = splitNChars(inventory_bytes);

      // fetching the gold of the player

      const gold_res = await fetch(
        `http://127.0.0.1:8080/state/value?path=/players/${player_address}/gold`
      );

      const gold_bytes = await gold_res.text();
      const gold = Number.parseInt(gold_bytes, 16);

      updatePlayer({
        x: 0,
        y: 0,
        inventory,
        gold
      });

      // Map -> Item
      const map_res = await fetch(
        "http://127.0.0.1:8080/state/value?path=/state/map"
      );

      const map_bytes = await map_res.text();
      // need to split the array to a list of string
      const map = splitNChars(map_bytes);

      // we can put new value to the map
      updateMap(map);

      // fetch the market place
      const marketplace_res = await fetch(
        "http://127.0.0.1:8080/state/subkeys?path=/market-place"
      );
      const sellers = await marketplace_res.json();

      // if there is no market place, create one
      let market_place = [];
      for (let index = 0; index < sellers.length; index++) {
        // get the address from the seller index
        const address = sellers[index];

        const items_res = await fetch(
          `http://127.0.0.1:8080/state/subkeys?path=/market-place/${address}`
        );
        const items = await items_res.json();

        // going to the items
        for (let indexItem = 0; indexItem < items.length; indexItem++) {
          const itemId = items[indexItem];

          // getting the price
          const price_res = await fetch(
            `http://127.0.0.1:8080/state/value?path=/market-place/${address}/${itemId}/value`
          );
          const price_bytes = await price_res.text();
          const price = Number.parseInt(price_bytes, 16);

          // push the sell into market place
          market_place.push({
            address,
            item: itemId,
            price
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
        <Map map={map} players={players} />
        {
          // Add buttons for player actions
        }
        <div className="buttons">
          {
            // Add buttons to switch between players
          }
          <PlayerAccounts setSecret={setSecret} />
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
        <Inventory map={map} player={player} drop={drop} sell={sell} />
        {
          // Display market place
        }
        <MarketPlace marketplace={marketplace} buy={buy} />
      </header>
    </div>
  );
};

export default App;
