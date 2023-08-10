import React, { Component } from "react";

const Map = ({ map, players }) => {
  return (
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
  );
};

export default Map;
