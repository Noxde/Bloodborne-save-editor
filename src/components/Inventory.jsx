import "./inventory.css";
import { useEffect, useRef, useState, useContext } from "react";
import { SaveContext } from "../context/context";
import { invoke } from "@tauri-apps/api";
import ReplaceScreen from "./ReplaceScreen";
import { getType } from "../utils/drawCanvas";
import FilterButtons from "./FilterButtons";
import FilterComponent from "./FilterComponent";
import EditUpgrade from "./EditUpgrade";
import AddScreen from "./AddScreen";
import { ImagesContext } from "../context/imagesContext";
import { useNavigate } from "react-router-dom";

function Inventory({ inv, isStorage }) {
  const inventoryRef = useRef(null);
  const [selected, setSelected] = useState(null);
  const selectedRef = useRef(null);
  const [quantity, setQuantity] = useState(0);
  const [hoverIndex, setHoverIndex] = useState(0);
  const [replaceScreen, setReplaceScreen] = useState(false);
  const [editScreen, setEditScreen] = useState(false);
  const [addScreen, setAddScreen] = useState(false);
  const [selectedFilter, setSelectedFilter] = useState("0");
  const nav = useNavigate();
  const {
    images: { items, backgrounds },
  } = useContext(ImagesContext);

  const { save, setSave } = useContext(SaveContext);

  useEffect(() => {
    const invCurrent = inventoryRef.current;
    function manageSelect(e) {
      const {
        target,
        srcElement: { nodeName },
      } = e;

      if (nodeName === "CANVAS") {
        const { item: itemRaw, index } = target.dataset;
        const item = JSON.parse(itemRaw);
        setHoverIndex(index);

        selectedRef.current = target;
        setSelected(item);
        setQuantity(item.amount);
      } else if (nodeName === "BUTTON") {
        const { index } = target.dataset;

        setSelected(null);

        setSelectedFilter((prev) => (prev === index ? "0" : index));
      }
    }

    if (save) {
      inventoryRef?.current?.addEventListener("click", manageSelect);
    }

    return () => {
      if (invCurrent) {
        invCurrent.removeEventListener("click", manageSelect);
      }
    };
  }, [inventoryRef, save]);

  useEffect(() => {
    if (!selected) {
      setHoverIndex(0);
      selectedRef.current = null;
    }
  }, [selected]);

  return (
    <>
      {/* Optional modal like screens */}
      {addScreen ? (
        <AddScreen
          type="item"
          setAddScreen={setAddScreen}
          isStorage={isStorage}
        />
      ) : null}
      {replaceScreen ? (
        <ReplaceScreen
          setSelected={setSelected}
          selected={selected}
          selectedRef={selectedRef}
          setReplaceScreen={setReplaceScreen}
          isStorage={isStorage}
        />
      ) : null}
      {editScreen ? (
        <EditUpgrade
          setSelected={setSelected}
          selected={selected}
          selectedRef={selectedRef}
          setEditScreen={setEditScreen}
          isStorage={isStorage}
        />
      ) : null}
      {/* Inventory */}
      <div ref={inventoryRef} style={{ overflowY: "scroll" }}>
        <FilterButtons selectedFilter={selectedFilter} />
        <div
          style={{
            position: "relative",
          }}
        >
          <div id="hover" style={{ top: `${(hoverIndex - 1) * 91}px` }}></div>
          <FilterComponent inventory={inv} selectedFilter={selectedFilter} />
        </div>
      </div>
      {/* Right side buttons */}
      <div className="editButtons">
        <div className="editQuantity">
          <input
            type="number"
            value={quantity || 0}
            max={isStorage ? 600 : 99}
            min={0}
            style={{ width: "120px" }}
            disabled={getType(selected?.article_type) !== "item" ? true : false}
            onChange={(e) => {
              const { value } = e.target;

              // Check if the item should be capped at 600 or not
              if (
                (!isStorage ||
                  (isStorage &&
                    selected.article_type !== "Material" &&
                    !["Quicksilver Bullets", "Blood Vial"].includes(
                      selected.info.item_name
                    ))) &&
                value > 99
              ) {
                setQuantity(99);
              } else if (isStorage && value > 600) {
                setQuantity(600);
              } else {
                setQuantity(parseInt(value));
              }
            }}
          />
          <button
            className="buttonBg"
            onClick={async () => {
              console.log(selected);
              const editedSave = await invoke("edit_quantity", {
                number: selected.number,
                id: selected.id,
                value: quantity,
                isStorage,
              });
              setSave(editedSave);

              const canvas = selectedRef.current;
              const ctx = canvas.getContext("2d");

              const itemImage = backgrounds["item.png"];

              ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
              await drawItem(ctx, selected.info, quantity, itemImage, items);
            }}
            disabled={
              getType(selected?.article_type) === "item" && quantity > 0
                ? false
                : true
            }
          >
            Set
          </button>
        </div>
        <button
          className="buttonBg inventory-btn"
          disabled={selected?.article_type === undefined}
          onClick={async () => {
            setReplaceScreen(true);
          }}
        >
          Replace
        </button>
        <button
          className="buttonBg inventory-btn"
          disabled={!selected?.upgrade_type}
          onClick={async () => {
            setEditScreen(true);
          }}
        >
          Edit
        </button>
        <button
          className="buttonBg inventory-btn"
          onClick={() => setAddScreen(true)}
        >
          Add
        </button>
        <button
          className="buttonBg inventory-btn"
          disabled={
            getType(selected?.article_type) !== "weapon" &&
            getType(selected?.article_type) !== "armor"
          }
          onClick={() =>
            nav("/equippedGems", {
              state: {
                selected,
                isStorage,
              },
            })
          }
        >
          Gems
        </button>
      </div>
    </>
  );
}

async function drawItem(ctx, item, amount, img, items) {
  const { x, y } = {
    x: 9,
    y: 6,
  };

  const size = 73;
  const { item_name: name, item_img: image, item_desc: note } = item;

  const thumbnail = items[image];

  ctx.font = "18px Reim";
  ctx.drawImage(img, 0, 0);
  ctx.drawImage(thumbnail, x, y, x + size, y + size);

  // Set up text
  ctx.shadowBlur = 3;
  ctx.shadowOffsetX = 0;
  ctx.shadowOffsetY = 2;
  ctx.shadowColor = "black";
  ctx.fillStyle = "#ab9e87";
  ctx.fillText(name, 107, 28);
  ctx.fillText(note, 104, 69);

  ctx.font = "24px Reim";
  ctx.fillStyle = "#FFFF";
  if (amount > 9) {
    ctx.fillText(amount, 60, 85);
  } else {
    ctx.fillText(amount, 75, 83);
  }
}

function loadImage(url) {
  return new Promise((resolve) => {
    let imageObj = new Image();
    imageObj.onload = () => resolve(imageObj);
    imageObj.src = url;
  });
}

export default Inventory;
