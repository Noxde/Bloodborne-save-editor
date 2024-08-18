import { useEffect, useContext, useState } from "react";
import Item from "./Item";
import { invoke } from "@tauri-apps/api";
import { drawCanvas, getType } from "../utils/drawCanvas";
import { SaveContext } from "../context/context";

function ReplaceScreen({
  setSelected,
  selected,
  setReplaceScreen,
  selectedRef,
}) {
  const [replacements, setReplacements] = useState(null);
  const [replacement, setReplacement] = useState(null);
  const [hoverIndex, setHoverIndex] = useState(0);
  const { setSave } = useContext(SaveContext);

  useEffect(() => {
    switch (getType(selected.article_type)) {
      case "weapon":
        invoke("return_weapons").then((weps) => {
          setReplacements(
            Object.keys(weps)
              .map((x) => {
                return Object.keys(weps[x]).map((y) => {
                  return {
                    id: parseInt(y),
                    article_type: `${x.at(0).toUpperCase() + x.slice(1)}`,
                    info: {
                      item_name: weps[x][y]["item_name"],
                      item_img: weps[x][y]["item_img"],
                      item_desc: weps[x][y]["item_desc"],
                      extra_info: {
                        damage: weps[x][y]["damage"],
                      },
                    },
                  };
                });
              })
              .flat()
          );
        });
        break;
      case "chalice":
      case "key":
      case "item":
        invoke("return_items").then((items) => {
          setReplacements(
            Object.keys(items)
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

                  if (getType(selected.article_type) == "chalice") {
                    item.info.extra_info = {
                      depth: items[x][y]["depth"],
                      area: items[x][y]["area"],
                    };
                  }

                  return item;
                });
              })
              .flat()
          );
        });
        break;
      case "armor":
        invoke("return_armors").then((armors) => {
          setReplacements(
            Object.keys(armors)
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
              .flat()
          );
        });
        break;
      default:
        break;
    }
  }, []);

  async function handleConfirm() {
    try {
      const edited = await invoke("transform_item", {
        id: selected.id,
        newId: parseInt(replacement.id),
        articleType: selected.article_type,
      });

      setSave(edited);

      console.log(selected.index);
      const newJson = {
        index: selected.index,
        amount: selected.amount,
        ...replacement,
      };
      const ctx = selectedRef.current.getContext("2d");
      selectedRef.current.dataset.itemId = parseInt(replacement.id);
      selectedRef.current.dataset.item = JSON.stringify(newJson);

      setSelected(newJson);
      ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
      await drawCanvas(ctx, newJson);

      console.log(replacement);
      setReplaceScreen(false);
    } catch (error) {
      console.log(error);
    }
  }

  return (
    <div id="replaceScreen">
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          padding: "0 3rem",
        }}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "space-between",
            alignItems: "start",
          }}
        >
          {/* To replace */}
          <div>
            <span>Replacing:</span>
            <Item item={selected} isSmall={true} />
          </div>
          <div>
            <button
              onClick={() => setReplaceScreen(false)}
              style={{ marginRight: "50px" }}
              id="cancelReplace"
            >
              Cancel
            </button>
            <button id="confirmReplace" onClick={handleConfirm}>
              Confirm
            </button>
          </div>
        </div>
        {/* List and input */}
        <div>
          <div>
            Select a new{" "}
            {getType(selected.article_type) === "key" ||
            getType(selected.article_type) === "chalice"
              ? "item"
              : getType(selected.article_type)}
            <input
              style={{ display: "block", width: "100%", fontSize: "1.2rem" }}
              type="text"
              placeholder="Chikage"
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
              style={{ top: `${hoverIndex * 91}px` }}
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
      </div>
    </div>
  );
}

export default ReplaceScreen;
