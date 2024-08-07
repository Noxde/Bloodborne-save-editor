import { useEffect, useRef, useState } from "react";
import items from "../items.json";
import weapons from "../weapons.json";
import armors from "../armors.json";

function Item({ item }) {
  const { id, article_type: type, amount } = item;
  const canvasRef = useRef(null);
  const [drawn, setDrawn] = useState(false);

  useEffect(() => {
    const canvas = canvasRef?.current;
    if (!canvas) return;

    const ctx = canvasRef.current.getContext("2d");
    let found;

    switch (type.toLowerCase()) {
      case "weapon":
        for (let k in weapons) {
          found = Object.keys(weapons[k]).find((x) => x == id);
          if (found) {
            found = weapons[k][found];
            break;
          }
        }

        if (found) {
          loadImage(process.env.PUBLIC_URL + "/assets/itemsBg/weapon.png").then(
            (weaponImage) => {
              drawWeapon(ctx, found, weaponImage);
              setDrawn(true);
            }
          );
        }
        break;

      case "item":
        let kind;
        for (let k in items) {
          found = Object.keys(items[k]).find((x) => x == id);
          if (found) {
            found = items[k][found];
            kind = k;
            break;
          }
        }

        if (found) {
          if (kind == "chalices") {
            loadImage(
              process.env.PUBLIC_URL + "/assets/itemsBg/chalice.png"
            ).then((chaliceImag) => {
              drawChalice(ctx, found, chaliceImag);
              setDrawn(true);
            });
          } else {
            loadImage(process.env.PUBLIC_URL + "/assets/itemsBg/item.png").then(
              (itemImage) => {
                drawItem(ctx, found, amount, itemImage);
                setDrawn(true);
              }
            );
          }
        }
        break;

      case "armor":
        found = Object.keys(armors).find((x) => x == id);
        if (found) {
          found = armors[found];
        }

        if (found) {
          loadImage(process.env.PUBLIC_URL + "/assets/itemsBg/armor.png").then(
            (armorImage) => {
              drawArmor(ctx, found, armorImage);
              setDrawn(true);
            }
          );
        }
        break;

      default:
        break;
    }
  }, [type, id]);

  return (
    <canvas
      data-item-id={id}
      width={795}
      height={90}
      ref={canvasRef}
      style={{ display: drawn ? "block" : "none" }}
    ></canvas>
  );
}

async function drawChalice(ctx, chalice, img) {
  const { x, y } = {
    x: 9,
    y: 6,
  };

  const size = 73;
  const { item_name: name, item_img: image, depth, area } = chalice;

  const thumbnail = await loadImage(image);

  ctx.font = "18px Reim";
  ctx.drawImage(img, 0, 0);
  ctx.drawImage(thumbnail, x, y, x + size, y + size);

  const margin = 100;
  // Draw numbers
  ctx.fillStyle = "#b8b7ad";
  ctx.fillText(depth, 145, 77);
  ctx.fillText(area, margin * 2 + 37, 77);

  // Set up text
  ctx.shadowBlur = 3;
  ctx.shadowOffsetX = 0;
  ctx.shadowOffsetY = 2;
  ctx.shadowColor = "black";
  ctx.fillStyle = "#ab9e87";
  ctx.fillText(name, 107, 28);
}

async function drawArmor(ctx, armor, img) {
  const { x, y } = {
    x: 9,
    y: 6,
  };
  const size = 73;
  const { name, image, physicalDefense, elementalDefense } = armor;
  const { physical, blunt, thrust, blood } = physicalDefense;
  const { arcane, fire, bolt } = elementalDefense;

  const thumbnail = await loadImage(image);

  ctx.font = "18px Reim";
  ctx.drawImage(img, 0, 0);
  ctx.drawImage(thumbnail, x, y, x + size, y + size);

  const margin = 100;
  // Draw numbers
  ctx.fillStyle = "#b8b7ad";
  ctx.fillText(physical, 137, 77);
  ctx.fillText(blunt, margin * 2 + 37, 77);
  ctx.fillText(thrust, margin * 3 + 37, 77);
  ctx.fillText(blood, margin * 4 + 37, 77);
  ctx.fillText(arcane, margin * 5 + 37, 77);
  ctx.fillText(fire, margin * 6 + 37, 77);
  ctx.fillText(bolt, margin * 7 + 37, 77);

  const hover = await loadImage(process.env.PUBLIC_URL + "/assets/hover.png");

  // Set up text
  ctx.shadowBlur = 3;
  ctx.shadowOffsetX = 0;
  ctx.shadowOffsetY = 2;
  ctx.shadowColor = "black";
  ctx.fillStyle = "#ab9e87";
  ctx.fillText(name, 107, 28);
  ctx.drawImage(hover, 0, 0);
}

async function drawWeapon(ctx, weapon, img) {
  const { x, y } = {
    x: 9,
    y: 6,
  };

  const size = 73;
  const { name, image, damage } = weapon;
  const { physical, blood, arcane, fire, bolt } = damage;

  const thumbnail = await loadImage(image);

  ctx.font = "18px Reim";
  ctx.drawImage(img, 0, 0);
  ctx.drawImage(thumbnail, x, y, x + size, y + size);

  const margin = 100;
  // Draw numbers
  ctx.fillStyle = "#b8b7ad";
  ctx.fillText(physical, 137, 77);
  ctx.fillText(blood, margin * 2 + 37, 77);
  ctx.fillText(arcane, margin * 3 + 37, 77);
  ctx.fillText(fire, margin * 4 + 37, 77);
  ctx.fillText(bolt, margin * 5 + 37, 77);

  // Set up text
  ctx.shadowBlur = 3;
  ctx.shadowOffsetX = 0;
  ctx.shadowOffsetY = 2;
  ctx.shadowColor = "black";
  ctx.fillStyle = "#ab9e87";
  ctx.fillText(name, 107, 28);
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

export default Item;
