import { useContext, useEffect, useState } from "react";
import Item from "./Item";
import { ItemsContext } from "../context/itemsContext";

/**
 *
 * @param {Object} props
 * @param {"item" | "armor" | "weapon"} props.type
 * @param {Function} props.onChange
 * @returns
 */
function SearchAllitems({ type, onChange, title }) {
  const [search, setSearch] = useState("");
  const [replacements, setReplacements] = useState(null);
  const [back, setBack] = useState(null);
  const [hoverIndex, setHoverIndex] = useState(null);
  const { weapons, items, armors, all } = useContext(ItemsContext);

  useEffect(() => {
    setHoverIndex(null);
    if (search) {
      setReplacements(
        back.filter((x) =>
          x.info.item_name.toLowerCase().includes(search.toLowerCase())
        )
      );
    } else {
      setReplacements(back);
    }
  }, [search]);

  console.log(type);

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
        setReplacements(items.filter((y) => y.article_type === type));
        setBack(items.filter((y) => y.article_type === type));
        break;
      case "armor":
        setReplacements(armors);
        setBack(armors);
        break;
      default:
        setReplacements(all);
        setBack(all);
        break;
    }
  }, []);

  return (
    <div style={{ width: "534px", maxHeight: "calc(100% - 149px)" }}>
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
          placeholder="Search"
        />
      </div>
      {/* List of items */}
      <div
        style={{
          position: "relative",
          overflowY: "scroll",
          // maxHeight: "calc(100% - 111px)",
          height: "100%",
        }}
      >
        <div
          id="hoverReplacement"
          style={{
            display: hoverIndex == null ? "none" : "block",
            top: `${hoverIndex * 91}px`,
          }}
        ></div>

        {replacements?.map((x, i) => (
          <Item
            onClick={() => {
              if (typeof onChange == "function") {
                onChange(x);
              }
              setHoverIndex(i);
            }}
            key={i}
            index={i + 1}
            item={x}
            isSmall={true}
          />
        ))}
      </div>
    </div>
  );
}

export default SearchAllitems;
