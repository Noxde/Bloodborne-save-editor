import React, { useState, useContext, useEffect } from "react";
import { SaveContext } from "../../context/context";
import Item from "../../components/Item";
import { Virtuoso } from "react-virtuoso";

function FilterComponent({ inventory, selectedFilter = 0, selectedIndex }) {
  const filters = [
    "Consumable",
    "Material",
    "Key",
    "RightHand",
    "LeftHand",
    "Armor",
    "Gem",
    "Rune",
    "Chalice",
  ];
  const { save } = useContext(SaveContext);

  const { articles, upgrades } = inventory;
  const all = { ...articles, ...upgrades };
  const [items, setItems] = useState(
    Object.keys(all)
      .map((x) => all[x])
      .flat(),
  );

  useEffect(() => {
    if (selectedFilter != 0) {
      setItems(all[filters[selectedFilter - 1]]);
    } else {
      setItems(
        Object.keys(all)
          .map((x) => all[x])
          .flat(),
      );
    }
  }, [selectedFilter, save]);

  return items?.length ? (
    <Virtuoso
      data={items}
      height={"100%"}
      itemContent={(i, item) => <Item index={i + 1} item={item} />}
      overscan={{
        main: 900,
        reverse: 900,
      }}
      fixedItemHeight={91}
    />
  ) : null;
}

export default FilterComponent;
