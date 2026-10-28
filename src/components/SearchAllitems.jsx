import { useContext, useEffect, useState, useRef } from "react";
import Item from "./Item";
import { ItemsContext } from "../context/itemsContext";
import { Virtuoso } from "react-virtuoso";
import "../pages/inventory/inventory.css";

/**
 *
 * @param {Object} props
 * @param {"item" | "armor" | "weapon" | "key" | "all"} props.type
 * @param {Function} props.onChange
 * @returns
 */
function SearchAllitems({ type, onChange, title }) {
  const [search, setSearch] = useState("");
  const [replacements, setReplacements] = useState(null);
  const [back, setBack] = useState(null);
  const { weapons, items, armors, all } = useContext(ItemsContext);
  const [selectedIndex, setSelectedIndex] = useState(null);

  useEffect(() => {
    setSelectedIndex(null);
    console.log("search effect");
    if (search) {
      setReplacements(
        back.filter((x) =>
          x.info.item_name.toLowerCase().includes(search.toLowerCase()),
        ),
      );
    } else {
      setReplacements(back);
    }
  }, [search]);

  useEffect(() => {
    switch (type) {
      case "weapon":
        setReplacements(weapons);
        setBack(weapons);
        break;
      case "chalice":
      case "item":
        setReplacements(items);
        setBack(items);
        break;
      case "key":
        setReplacements(
          items.filter((y) => y.article_type.toLowerCase() === type),
        );
        setBack(items.filter((y) => y.article_type.toLowerCase() === type));
        break;
      case "armor":
        setReplacements(armors);
        setBack(armors);
        break;
      case "all":
      default:
        setReplacements(all);
        setBack(all);
        break;
    }
  }, []);

  return (
    <div style={{ width: "534px", height: "calc(100% - 149px)" }}>
      <div>
        {title ? title : null}

        <input
          onChange={(e) => {
            const { target } = e;
            setSearch(target.value);
          }}
          value={search}
          style={{ display: "block", width: "100%", fontSize: "1.2rem" }}
          type="text"
          placeholder={type}
        />
      </div>
      {/* List of items */}
      <div
        style={{
          position: "relative",
          // maxHeight: "calc(100% - 111px)",
          height: "100%",
        }}
      >
        <Virtuoso
          data={replacements}
          height={"500px"}
          itemContent={(index, item) => (
            <Item
              className={selectedIndex === index ? "selectedItem small" : ""} // FIXME: El select con esto me clava un re-render y parpadea lo que seleccionas
              onClick={() => {
                if (typeof onChange == "function") {
                  onChange(item);
                }
                setSelectedIndex(index);
              }}
              key={index}
              index={index + 1}
              item={item}
              isSmall={true}
            />
          )}
          overscan={{
            main: 900,
            reverse: 900,
          }}
          fixedItemHeight={91}
        />
      </div>
    </div>
  );
}

export default SearchAllitems;
