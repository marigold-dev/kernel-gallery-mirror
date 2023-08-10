import React from "react";

const MarketPlace = ({ marketplace, buy }) => {
  return (
    <div style={{ marginTop: "24px" }}>
      <div>Marketplace:</div>
      {marketplace.map((item_to_sell, i) => {
        const { address, price, item } = item_to_sell;
        const item_name = item === "01" ? "sword" : "potion";

        return (
          <div>
            {address} - {item_name} - {price}{" "}
            <button onClick={buy(address, Number.parseInt(item))}>buy</button>
          </div>
        );
      })}
    </div>
  );
};

export default MarketPlace;
