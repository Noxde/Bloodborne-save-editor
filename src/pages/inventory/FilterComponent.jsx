import { useState, useContext, useEffect } from "react";
import { SaveContext } from "../../context/context";
import Item from "../../components/Item";

function FilterComponent({ inventory, selectedFilter = 0 }) {
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

  // upgrades?.Gem?.sort((a, b) => {
  //   // Sort by shape (alphabetical order)
  //   if (a.shape < b.shape) return -1;
  //   if (a.shape > b.shape) return 1;

  //   // If shapes are the same, sort by rating (numerical order)
  //   if (a.rating !== b.rating) return a.rating - b.rating;

  //   // If shapes and ratings are the same, sort by the first effect (alphabetical order)
  //   if (a.info.effect < b.info.effect) return -1;
  //   if (a.info.effect > b.info.effect) return 1;

  //   // If all are the same, return 0
  //   return 0;
  // });

  const all = { ...articles, ...upgrades };
  const [items, setItems] = useState(
    Object.keys(all)
      .map((x) => all[x])
      .flat()
  );

  useEffect(() => {
    if (selectedFilter != 0) {
      setItems(all[filters[selectedFilter - 1]]);
    } else {
      setItems(
        Object.keys(all)
          .map((x) => all[x])
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
