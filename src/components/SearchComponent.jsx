import { useContext, useEffect, useState } from "react";
import { getType } from "../utils/drawCanvas";
import Item from "./Item";
import { ItemsContext } from "../context/itemsContext";

function SearchComponent({ selected, setReplacement }) {
  const [search, setSearch] = useState("");
  const [replacements, setReplacements] = useState(null);
  const [back, setBack] = useState(null);
  const [hoverIndex, setHoverIndex] = useState(null);
  const { weapons, items, armors } = useContext(ItemsContext);

  useEffect(() => {
    setReplacement(null);
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

  useEffect(() => {
    switch (getType(selected.article_type)) {
      case "weapon":
        setReplacements(
          weapons.filter((y) => y.article_type == selected.article_type)
        );
        setBack(weapons);
        break;
      case "chalice":
      case "key":
      case "item":
        setReplacements(
          items.filter((y) => y.article_type == selected.article_type) // TODO: Fix this to be able to replace any kind of item
        );
        setBack(items.filter((y) => y.article_type == selected.article_type));
        break;
      case "armor":
        setReplacements(armors);
        setBack(armors);
        break;
      default:
        break;
    }
  }, []);

  return (
    <div style={{ width: "534px" }}>
      <div>
        Select a new{" "}
        {getType(selected.article_type) === "key" ||
        getType(selected.article_type) === "chalice"
          ? "item"
          : getType(selected.article_type)}
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
          maxHeight: "400px",
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
              setReplacement(x);
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

export default SearchComponent;
