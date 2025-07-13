import { invoke } from "@tauri-apps/api";
import { useEffect, useState, useContext } from "react";
import { SaveContext } from "../context/context";

function ShapeSelector({ shape, isStorage, article, setArticle, slotIndex }) {
  const [open, setOpen] = useState(false);
  const [selected, setSelected] = useState(shape);
  const { setSave } = useContext(SaveContext);
  const shapes = ["Closed", "Radial", "Triangle", "Waning", "Circle"];

  useEffect(() => {
    setArticle((prev) => {
      const copy = JSON.parse(JSON.stringify(prev));

      copy.slots[slotIndex].shape = selected;

      return copy;
    });

    invoke("edit_slot", {
      isStorage,
      articleType: article.article_type,
      articleIndex: article.index,
      slotIndex,
      newShape: selected,
    }).then((save) => setSave(save));
  }, [selected]);

  return (
    <>
      <div
        style={{
          position: "absolute",
          width: "50px",
          height: "50px",
          right: "1px",
          top: "1px",
          cursor: "pointer",
          backgroundImage: "url(/assets/shape_bg.png)",
          backgroundSize: "contain",
        }}
        onClick={() => setOpen((prev) => !prev)}
      >
        {/* If the slot is not closed, show the selected shape. Else, show an empty div */}
        {selected !== "Closed" ? (
          <img
            style={{
              display: "block",
              position: "relative",
              top: 1,
              left: 1,
              zIndex: 10,
            }}
            src={`/assets/${selected.toLowerCase()}.png`}
            width="48px"
            alt=""
          />
        ) : (
          <div
            style={{
              width: "48px",
              aspectRatio: "1/1",
            }}
          ></div>
        )}

        {open ? (
          <div
            style={{
              position: "absolute",
              userSelect: "none",
            }}
          >
            {shapes
              .filter((x) => x !== selected)
              .map((x) => (
                <img
                  style={{
                    position: "relative",
                    display: "block",
                    zIndex: 10,
                  }}
                  src={`/assets/${
                    x === "Closed" ? "shape_bg" : x.toLowerCase()
                  }.png`}
                  width="48px"
                  alt=""
                  onClick={() => setSelected(x)}
                />
              ))}
          </div>
        ) : null}
      </div>
    </>
  );
}

export default ShapeSelector;
