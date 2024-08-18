import { useState, useContext, useEffect } from "react";
import { SaveContext } from "../context/context";
import Item from "./Item";

function FilterComponent({ selectedFilter = 0 }) {
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
  const {
    inventory: { articles },
  } = save;
  const [items, setItems] = useState(
    Object.keys(articles)
      .map((x) => articles[x])
      .flat()
  );

  useEffect(() => {
    if (selectedFilter != 0) {
      setItems(articles[filters[selectedFilter - 1]]);
    } else {
      setItems(
        Object.keys(articles)
          .map((x) => articles[x])
          .flat()
      );
    }
  }, [selectedFilter, save]);

  return (
    <>
      {items?.map((x, i) => (
        <Item key={i} index={i + 1} item={x} />
      ))}
    </>
  );
}

export default FilterComponent;
