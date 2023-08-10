import React, { Component } from "react";
import { ItemType } from "./item";

const Inventory = ({ player, drop, sell }) => {
  return (
    <div>
      <div>Inventory:</div>
      {player && player.gold && <div>Gold: {player.gold}</div>}
      {player.inventory.map((item, i) => {
        // matching the items as before for display
        switch (item) {
          case "01":
            return (
              <ItemType i={i} drop={drop} sell={sell} itemType={"sword"} />
            );
          case "02":
            return (
              <ItemType i={i} drop={drop} sell={sell} itemType={"potion"} />
            );
          default:
            return null;
        }
      })}
    </div>
  );
};

export default Inventory;
