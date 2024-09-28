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

  if (item?.upgrade_type == "Gem") {
    loadImage(process.env.PUBLIC_URL + "/assets/itemsBg/gem.png").then(
      (gemImage) => {
        drawGem(ctx, item, gemImage);
      }
    );
  } else if (item?.upgrade_type == "Rune") {
    loadImage(process.env.PUBLIC_URL + "/assets/itemsBg/item.png").then(
      (runeImage) => {
        drawRune(ctx, item, runeImage);
      }
    );
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

async function drawRune(ctx, rune, img) {
  const { x, y } = {
    x: 9,
    y: 4.8,
  };

  const size = 73;
  const {
    info: { name, rating, note },
    shape,
  } = rune;

  const path = getRunePath(name, shape, rating);

  const thumbnail = await loadImage(path);

  ctx.font = "18px Reim";
  ctx.shadowBlur = 0;
  ctx.shadowOffsetY = 0;

  ctx.drawImage(img, 0, 0);
  ctx.drawImage(thumbnail, x, y, x + size, y + size + 2);

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
}

async function drawGem(ctx, gem, img) {
  const { x, y } = {
    x: 9,
    y: 6,
  };

  const size = 73;
  const {
    effects,
    info: { name, level, rating },
    shape,
    source,
  } = gem;

  const uniqueGem = getUnique(effects[0][0], shape, source);
  const thumbnail = await loadImage(
    getGemPath(effects, shape, level, uniqueGem)
  );

  ctx.font = "20px Reim";
  ctx.drawImage(img, 0, 0);
  ctx.drawImage(thumbnail, x, y, x + size, y + size);

  const margin = 100;
  // Draw numbers
  ctx.fillStyle = "#b8b7ad";
  ctx.fillText(rating, 135, 77);
  ctx.fillText(shape, margin * 2 + 27, 77);

  // Set up text
  ctx.shadowBlur = 3;
  ctx.shadowOffsetX = 0;
  ctx.shadowOffsetY = 2;
  ctx.shadowColor = "black";
  ctx.fillStyle = "#ab9e87";
  ctx.fillText(
    uniqueGem
      ? uniqueGem.name
      : [2147633649, 2147633648, 2147633650].includes(source)
      ? "?GemName?"
      : name,
    107,
    28
  );
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

function getRunePath(name, shape, rating) {
  const normalized = name.toLowerCase().replaceAll(" ", "_");

  if (shape === "Oath") {
    return `/assets/runes/oath/${normalized}.png`;
  } else {
    return `/assets/runes/${removeRunePrefix(normalized)}/${rating}.png`;
  }
}

function removeRunePrefix(name) {
  return name
    .replaceAll(/great_|arcane_|dissipating_|stunning_|clear_|fading_/g, "")
    .trim();
}

function isCursed(effects) {
  return effects.some(
    ([_, name]) =>
      name.includes("-") ||
      name.includes("Increases stamina") ||
      name.includes("DOWN")
  );
}

function getUnique(primaryEffect, shape, source) {
  if (shape === "Droplet") {
    if (primaryEffect === 3143408 && source === 2147633649) {
      return { image: "tear", name: "Tear Blood Gem" };
    } else if (primaryEffect === 3126204 && source === 2147633648) {
      return { image: "brooch", name: "Red Blood Gem" };
    }
  } else if (shape === "Radial") {
    if (primaryEffect === 3133407 && source === 2147633650) {
      return { image: "gold", name: "Gold Blood Gem" };
    }
  }
}

function getGemPath(effects, shape, level, unique) {
  if (unique) return `/assets/gems/unique/${unique.image}.png`;

  const color = getGemColor(effects[0][1]);
  const cursed = isCursed(effects);
  return `/assets/gems/${shape.toLowerCase()}/${color}/${
    cursed ? "cursed_" : ""
  }${level}.png`;
}

function getGemColor(primaryEffect) {
  const lowerCaseEffect = primaryEffect.toLowerCase();

  switch (true) {
    case /vs beasts|blood/.test(lowerCaseEffect):
      return "blue";
    case /(?:slow|rapid) poison effect/.test(lowerCaseEffect):
      return "purple";
    case /bolt/.test(lowerCaseEffect):
      return "yellow";
    case /fire|vs the kin/.test(lowerCaseEffect):
      return "orange";
    case /charge atks up|stamina cost|phys. up|boosts rally|hp continues|wpn durability/.test(
      lowerCaseEffect
    ):
      return "green";
    case /arcane/.test(lowerCaseEffect):
      return "white";
    case /physical|skl|str|thrust|blunt|atk/.test(lowerCaseEffect):
      return "red";
    default:
      return null; // or some default value
  }
}

export {
  getType,
  drawCanvas,
  getGemColor,
  getRunePath,
  getGemPath,
  isCursed,
  getUnique,
};
