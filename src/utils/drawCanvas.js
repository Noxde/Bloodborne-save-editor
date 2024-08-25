async function drawCanvas(ctx, item, isSmall = false) {
  const { article_type, amount, info } = item;
  const type = getType(article_type);

  if (isSmall) {
    loadImage(process.env.PUBLIC_URL + "/assets/itemsBg/item_small.png").then(
      (itemImg) => {
        drawItem(ctx, info, "", itemImg);
      }
    );
    return;
  }

  switch (type) {
    case "weapon":
      loadImage(process.env.PUBLIC_URL + "/assets/itemsBg/weapon.png").then(
        (weaponImage) => {
          drawWeapon(ctx, info, weaponImage);
        }
      );
      break;

    case "item":
    case "key":
      loadImage(process.env.PUBLIC_URL + "/assets/itemsBg/item.png").then(
        (itemImage) => {
          drawItem(ctx, info, type === "key" ? "" : amount, itemImage);
        }
      );
      break;

    case "chalice":
      loadImage(process.env.PUBLIC_URL + "/assets/itemsBg/chalice.png").then(
        (chaliceImage) => {
          drawChalice(ctx, info, chaliceImage);
        }
      );
      break;

    case "armor":
      loadImage(process.env.PUBLIC_URL + "/assets/itemsBg/armor.png").then(
        (armorImage) => {
          drawArmor(ctx, info, armorImage);
        }
      );
      break;

    default:
      break;
  }
}

async function drawChalice(ctx, chalice, img) {
  const { x, y } = {
    x: 9,
    y: 6,
  };

  const size = 73;
  const {
    item_name: name,
    item_img: image,
    extra_info: { depth, area },
  } = chalice;

  const thumbnail = await loadImage("/assets/itemImages/" + image);

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
  const {
    item_name: name,
    item_img: image,
    extra_info: { physicalDefense, elementalDefense },
  } = armor;
  const { physical, blunt, thrust, blood } = physicalDefense;
  const { arcane, fire, bolt } = elementalDefense;

  const thumbnail = await loadImage("/assets/itemImages/" + image);

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

  // const hover = await loadImage(process.env.PUBLIC_URL + "/assets/hover.png");

  // Set up text
  ctx.shadowBlur = 3;
  ctx.shadowOffsetX = 0;
  ctx.shadowOffsetY = 2;
  ctx.shadowColor = "black";
  ctx.fillStyle = "#ab9e87";
  ctx.fillText(name, 107, 28);
  // ctx.drawImage(hover, 0, 0);
}

async function drawWeapon(ctx, weapon, img) {
  const { x, y } = {
    x: 9,
    y: 6,
  };

  const size = 73;
  const {
    item_name: name,
    item_img: image,
    extra_info: { damage, upgrade_level: upgrade, imprint },
  } = weapon;
  const { physical, blood, arcane, fire, bolt } = damage;
  const finalName = `${imprint ? imprint + " " : ""}${name}${
    upgrade > 0 ? " +" + upgrade : ""
  }`;

  const thumbnail = await loadImage("/assets/itemImages/" + image);

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
  ctx.fillText(finalName, 107, 28);
}

async function drawItem(ctx, item, amount, img) {
  const { x, y } = {
    x: 9,
    y: 6,
  };

  const size = 73;
  const { item_name: name, item_img: image, item_desc: note } = item;

  const thumbnail = await loadImage("/assets/itemImages/" + image);

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
  ctx.fillStyle = "#dbd9d5";
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
    imageObj.onerror = () => {
      // If the image fails to load, use the default image
      imageObj.src = "/assets/itemImages/empty.png";
    };
    imageObj.src = url;
  });
}

function getType(type) {
  if (!type) return "";
  switch (type.toLowerCase()) {
    case "consumable":
    case "material":
      return "item";
    case "lefthand":
    case "righthand":
      return "weapon";
    default:
      return type.toLowerCase();
  }
}

export { getType, drawCanvas };
