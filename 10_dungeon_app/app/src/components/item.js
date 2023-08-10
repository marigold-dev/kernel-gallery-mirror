import React from "react";

export const ItemType = ({ i, drop, sell, itemType }) => {
  return (
    <div className="item">
      <div key={i}>{itemType === "sword" ? "Sword" : "Potion"}</div>
      <button onClick={drop(i)}>drop</button>
      <button onClick={sell(i)}>sell</button>
    </div>
  );
};
