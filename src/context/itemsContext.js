import React, { createContext, useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api";

export const ItemsContext = createContext();

export const ItemsProvider = ({ children }) => {
  const [items, setItems] = useState({
    weapons: [],
    items: [],
    armors: [],
  });

  useEffect(() => {
    const fetchData = async () => {
      const weapons = await invoke("return_weapons");
      const items = await invoke("return_items");
      const armors = await invoke("return_armors");

      const transformedWeapons = Object.keys(weapons)
        .map((x) => {
          return Object.keys(weapons[x]).map((y) => {
            return {
              id: parseInt(y),
              article_type: `${x.at(0).toUpperCase() + x.slice(1)}`,
              info: {
                item_name: weapons[x][y]["item_name"],
                item_img: weapons[x][y]["item_img"],
                item_desc: weapons[x][y]["item_desc"],
                extra_info: {
                  damage: weapons[x][y]["damage"],
                },
              },
            };
          });
        })
        .flat();

      const transformedItems = Object.keys(items)
        .map((x) => {
          return Object.keys(items[x]).map((y) => {
            let item = {
              id: parseInt(y),
              article_type: `${x.at(0).toUpperCase() + x.slice(1)}`,
              info: {
                item_name: items[x][y]["item_name"],
                item_img: items[x][y]["item_img"],
                item_desc: items[x][y]["item_desc"],
              },
            };

            if (item.article_type === "Chalice") {
              item.info.depth = items[x][y]["depth"];
              item.info.area = items[x][y]["area"];
            }

            return item;
          });
        })
        .flat();

      const transformedArmors = Object.keys(armors)
        .map((x) => {
          return {
            id: parseInt(x),
            article_type: "Armor",
            info: {
              item_name: armors[x]["item_name"],
              item_img: armors[x]["item_img"],
              item_desc: armors[x]["item_desc"],
              extra_info: {
                physicalDefense: armors[x]["physicalDefense"],
                elementalDefense: armors[x]["elementalDefense"],
                resistance: armors[x]["resistance"],
                beasthood: armors[x]["beasthood"],
                type: armors[x]["type"],
              },
            },
          };
        })
        .flat();

      setItems({
        weapons: transformedWeapons,
        items: transformedItems,
        armors: transformedArmors,
      });
    };

    fetchData();
  }, []);

  return (
    <ItemsContext.Provider value={items}>{children}</ItemsContext.Provider>
  );
};
