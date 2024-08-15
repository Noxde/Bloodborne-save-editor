import "./inventory.css";
import { useEffect, useRef, useState, useContext } from "react";
import { SaveContext } from "../context/context";
import Item from "./Item";
import { invoke } from "@tauri-apps/api";
import ReplaceScreen from "./ReplaceScreen";
import { getType } from "../utils/drawCanvas";

function Inventory() {
  const inventoryRef = useRef(null);
  const [selected, setSelected] = useState(null);
  const selectedRef = useRef(null);
  const [quantity, setQuantity] = useState(0);
  const [replaceScreen, setReplaceScreen] = useState(false);
  const { save, setSave } = useContext(SaveContext);

  useEffect(() => {
    function manageSelect(e) {
      const {
        target,
        srcElement: { nodeName },
      } = e;
      console.log(nodeName);
      if (nodeName === "CANVAS") {
        // console.log(target.dataset.itemId);
        const { item: itemRaw } = target.dataset;
        const item = JSON.parse(itemRaw);

        setSelected(item);
        setQuantity(item.amount);
      }
    }

    if (save) {
      console.log(inventoryRef?.current);
      inventoryRef?.current?.addEventListener("click", manageSelect);
    }

    return () => {
      if (inventoryRef.current) {
        inventoryRef.current.removeEventListener("click", manageSelect); // Change to variable
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
        <div
          className="filters"
          style={{
            position: "sticky",
            top: 0,
            width: "795px",
            height: "90px",
            background: "grey",
            textAlign: "center",
          }}
        >
          {selected?.id}
        </div>
        {save.inventory.articles.map((x) => (
          <Item key={x.index} item={x} />
        ))}
      </div>
      <div className="editButtons">
        <div className="editQuantity">
          <input
            type="number"
            value={quantity || 0}
            disabled={selected?.article_type !== "Item" ? true : false}
            onChange={(e) => {
              setQuantity(parseInt(e.target.value));
            }}
          />
          <button
            onClick={async () => {
              const editedSave = await invoke("edit_quantity", {
                save: JSON.stringify(save),
                index: selected.index,
                value: quantity,
              });

              const canvas = document.querySelector(
                `canvas[data-item='${JSON.stringify(selected)}']`
              );
              console.log(canvas);
              console.log(selected);
              console.log(quantity);
              const ctx = canvas.getContext("2d");

              const itemImage = await loadImage(
                process.env.PUBLIC_URL + "/assets/itemsBg/item.png"
              );

              ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
              await drawItem(ctx, selected.info, quantity, itemImage);

              setSave(editedSave);
            }}
            disabled={
              selected?.article_type === "Item" && quantity > 0 ? false : true
            }
          >
            Set
          </button>
        </div>
        <button>test</button>
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
