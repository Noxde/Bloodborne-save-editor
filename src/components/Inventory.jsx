import "./inventory.css";
import { useEffect, useRef, useState, useContext } from "react";
import { SaveContext } from "../context/context";
import { invoke } from "@tauri-apps/api";
import ReplaceScreen from "./ReplaceScreen";
import { getType } from "../utils/drawCanvas";
import FilterButtons from "./FilterButtons";
import FilterComponent from "./FilterComponent";

function Inventory() {
  const inventoryRef = useRef(null);
  const [selected, setSelected] = useState(null);
  const selectedRef = useRef(null);
  const [quantity, setQuantity] = useState(0);
  const [hoverIndex, setHoverIndex] = useState(0);
  const [replaceScreen, setReplaceScreen] = useState(false);
  const [selectedFilter, setSelectedFilter] = useState("0");

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
        setHoverIndex(0);
        selectedRef.current = null;

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

  return (
    <>
      {replaceScreen ? (
        <ReplaceScreen
          setSelected={setSelected}
          selected={selected}
          selectedRef={selectedRef}
          setReplaceScreen={setReplaceScreen}
        />
      ) : null}
      <div
        ref={inventoryRef}
        style={{ position: "relative", overflowY: "scroll" }}
      >
        <FilterButtons selectedFilter={selectedFilter} />
        <div id="hover" style={{ top: `${hoverIndex * 91}px` }}></div>
        <FilterComponent selectedFilter={selectedFilter} />
      </div>
      <div className="editButtons">
        <div className="editQuantity">
          <input
            type="number"
            value={quantity || 0}
            max={99}
            min={0}
            style={{ width: "120px" }}
            disabled={getType(selected?.article_type) !== "item" ? true : false}
            onChange={(e) => {
              const { value } = e.target;
              if (value > 99) {
                setQuantity(99);
              } else {
                setQuantity(parseInt(value));
              }
            }}
          />
          <button
            className="buttonBg"
            onClick={async () => {
              const editedSave = await invoke("edit_quantity", {
                index: selected.index,
                id: selected.id,
                value: quantity,
              });
              setSave(editedSave);

              const canvas = selectedRef.current;
              const ctx = canvas.getContext("2d");

              const itemImage = await loadImage(
                process.env.PUBLIC_URL + "/assets/itemsBg/item.png"
              );

              ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
              await drawItem(ctx, selected.info, quantity, itemImage);
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
          className="buttonBg"
          disabled={!!!selected}
          onClick={async () => {
            setReplaceScreen(true);
          }}
          style={{ width: "200px", backgroundSize: "100% 100%" }}
        >
          Replace
        </button>
      </div>
    </>
  );
}

async function drawItem(ctx, item, amount, img) {
  const { x, y } = {
    x: 9,
    y: 6,
  };

  const size = 73;
  const { item_name: name, item_img: image, item_desc: note } = item;

  const thumbnail = await loadImage(image);

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
