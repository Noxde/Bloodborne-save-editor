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
  const [hoverIndex, setHoverIndex] = useState(0);
  const [replaceScreen, setReplaceScreen] = useState(false);
  const [selectedFilter, setSelectedFilter] = useState(null);

  const { save, setSave } = useContext(SaveContext);

  let index = 0;
  useEffect(() => {
    function manageSelect(e) {
      const {
        target,
        srcElement: { nodeName },
      } = e;
      console.log(nodeName);
      if (nodeName === "CANVAS") {
        const { item: itemRaw, index } = target.dataset;
        const item = JSON.parse(itemRaw);
        setHoverIndex(index);
        console.log(item);

        selectedRef.current = target;
        setSelected(item);
        setQuantity(item.amount);
      } else if (nodeName === "BUTTON") {
        const { index } = target.dataset;
        setSelectedFilter(index);
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
        <div className="filters">
          <div
            id="filterHover"
            style={{
              left: `${5}px`, // Still need to use selectedFilter
            }}
          ></div>
          {/* Could be changed with a component that builds the buttons */}
          {/* Should change the naming scheme to be iterable */}
          <button
            data-index={1}
            className="filters-button"
            style={{
              background: `url(${
                process.env.PUBLIC_URL + "/assets/filters/consumables.png"
              })`,
            }}
          ></button>
          <button
            data-index={2}
            className="filters-button"
            style={{
              background: `url(${
                process.env.PUBLIC_URL + "/assets/filters/materials.png"
              })`,
            }}
          ></button>
          <button
            data-index={3}
            className="filters-button"
            style={{
              background: `url(${
                process.env.PUBLIC_URL + "/assets/filters/key.png"
              })`,
            }}
          ></button>
          <button
            data-index={4}
            className="filters-button"
            style={{
              background: `url(${
                process.env.PUBLIC_URL + "/assets/filters/right_hand.png"
              })`,
            }}
          ></button>
          <button
            data-index={5}
            className="filters-button"
            style={{
              background: `url(${
                process.env.PUBLIC_URL + "/assets/filters/left_hand.png"
              })`,
            }}
          ></button>
          <button
            data-index={6}
            className="filters-button"
            style={{
              background: `url(${
                process.env.PUBLIC_URL + "/assets/filters/armor.png"
              })`,
            }}
          ></button>
          <button
            data-index={7}
            className="filters-button"
            style={{
              background: `url(${
                process.env.PUBLIC_URL + "/assets/filters/gems.png"
              })`,
            }}
          ></button>
          <button
            data-index={8}
            className="filters-button"
            style={{
              background: `url(${
                process.env.PUBLIC_URL + "/assets/filters/runes.png"
              })`,
            }}
          ></button>
          <button
            data-index={9}
            className="filters-button"
            style={{
              background: `url(${
                process.env.PUBLIC_URL + "/assets/filters/chalices.png"
              })`,
            }}
          ></button>
        </div>
        <div id="hover" style={{ top: `${hoverIndex * 91}px` }}></div>
        {Object.keys(save.inventory.articles)
          .map((x) =>
            save.inventory.articles[x].map((y) => {
              return <Item key={index++} index={index + 1} item={y} />;
            })
          )
          .flat()}
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
              setSave(editedSave);

              const canvas = document.querySelector(
                `canvas[data-item-id='${selected.id}']`
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
