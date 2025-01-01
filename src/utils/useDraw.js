import { useContext } from "react";
import { ImagesContext } from "../context/imagesContext";

function useDraw() {
  const { images } = useContext(ImagesContext);

  async function drawCanvas(ctx, item, isSmall = false, context = images) {
    const { info } = item;

    if (isSmall) {
      return drawItem(
        ctx,
        info,
        "",
        images.backgrounds["item_small.png"],
        context
      );
    }
    const itemBackground = images.backgrounds[getBackground(item)];
    drawArticle(ctx, item, itemBackground, context);
  }

  return drawCanvas;
}

export default useDraw;

function getBackground(item) {
  const type = getType(item.article_type);
  switch (type || item.upgrade_type) {
    case "Gem":
      return "gem.png";
    case "chalice":
      return "chalice.png";
    case "weapon":
      return "weapon.png";
    case "armor":
      return "armor.png";
    case "Rune":
    case "key":
    case "item":
      return "item.png";
  }
}

async function drawArticle(ctx, article, img, imgContext) {
  const { x, y } = {
    x: 9,
    y: 6,
  };

  const size = 73;
  const { article_type, amount, info } = article;
  let { item_name: name, item_desc: note, item_img: image } = info;
  const type = getType(article_type);
  name = name ?? (article?.upgrade_type !== "Gem" ? info.name : ""); // Check for gems and runes
  note = note ?? info.note ?? "";
  ctx.drawImage(img, 0, 0);

  if (image) {
    const thumbnail = imgContext.items[image || "empty.png"];
    ctx.drawImage(thumbnail, x, y, x + size, y + size);
  }

  // Set up text
  ctx.font = "18px Reim";
  ctx.shadowBlur = 3;
  ctx.shadowOffsetX = 0;
  ctx.shadowOffsetY = 2;
  ctx.shadowColor = "black";
  ctx.fillStyle = "#ab9e87";

  if (article?.upgrade_type) {
    handleUpgrades(ctx, article, { x, y, size });
  }
  if (type === "chalice") {
    handleChalice(ctx, article);
  }

  switch (type || article_type) {
    case "weapon":
      return handleWeapon(ctx, info);
    case "armor":
      return handleArmor(ctx, info);
    default:
      break;
  }

  ctx.fillText(name, 107, 28);
  ctx.fillText(note, 104, 69);

  if (type === "item" && type !== "key" && type !== "chalice") {
    ctx.font = "24px Reim";
    ctx.fillStyle = "#dbd9d5";
    if (amount > 99) {
      ctx.fillText(amount, 45, 83);
    } else if (amount > 9) {
      ctx.fillText(amount, 60, 85);
    } else {
      ctx.fillText(amount, 75, 83);
    }
  }
}

function handleChalice(ctx, chalice) {
  const {
    info: {
      extra_info: { depth, area },
    },
  } = chalice;
  const margin = 100;
  ctx.fillText(depth, 135, 77);
  ctx.fillText(area, margin * 2 + 27, 77);
}

function handleWeapon(ctx, weapon) {
  const {
    item_name: name,
    extra_info: { damage, upgrade_level: upgrade, imprint },
  } = weapon;
  const { physical, blood, arcane, fire, bolt } = damage;
  const finalName = `${imprint ? imprint + " " : ""}${name}${
    upgrade > 0 ? " +" + upgrade : ""
  }`;
  ctx.fillText(finalName, 107, 28);

  const margin = 100;
  // Draw numbers
  ctx.fillStyle = "#b8b7ad";
  ctx.fillText(physical, 137, 77);
  ctx.fillText(blood, margin * 2 + 37, 77);
  ctx.fillText(arcane, margin * 3 + 37, 77);
  ctx.fillText(fire, margin * 4 + 37, 77);
  ctx.fillText(bolt, margin * 5 + 37, 77);
}

function handleArmor(ctx, armor) {
  const {
    item_name: name,
    extra_info: { physicalDefense, elementalDefense },
  } = armor;
  const { physical, blunt, thrust, blood } = physicalDefense;
  const { arcane, fire, bolt } = elementalDefense;
  ctx.fillText(name, 107, 28);

  const margin = 100;

  ctx.fillStyle = "#b8b7ad";
  ctx.fillText(physical, 137, 77);
  ctx.fillText(blunt, margin * 2 + 37, 77);
  ctx.fillText(thrust, margin * 3 + 37, 77);
  ctx.fillText(blood, margin * 4 + 37, 77);
  ctx.fillText(arcane, margin * 5 + 37, 77);
  ctx.fillText(fire, margin * 6 + 37, 77);
  ctx.fillText(bolt, margin * 7 + 37, 77);
}

async function handleUpgrades(ctx, upgrade, { x, y, size }) {
  if (upgrade.upgrade_type === "Gem") {
    const {
      effects,
      info: { name, level, rating },
      shape,
      source,
    } = upgrade;

    const uniqueGem = getUnique(effects[0][0], shape, source);
    const thumbnail = await loadImage(
      getGemPath(effects, shape, level, uniqueGem)
    );

    ctx.font = "20px Reim";
    ctx.drawImage(thumbnail, x, y, x + size, y + size);

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

    const margin = 100;
    // Draw numbers
    ctx.fillStyle = "#b8b7ad";
    ctx.fillText(rating, 135, 77);
    ctx.fillText(shape, margin * 2 + 27, 77);
  } else {
    const {
      info: { name, rating },
      shape,
    } = upgrade;

    const path = getRunePath(name, shape, rating);

    const thumbnail = await loadImage(path);

    ctx.drawImage(thumbnail, x, 4.8, x + size, 4.8 + size + 2);
  }
}

async function drawItem(ctx, item, amount, img, context) {
  const { x, y } = {
    x: 9,
    y: 6,
  };

  const size = 73;
  const { item_name: name, item_img: image, item_desc: note } = item;

  // const thumbnail = await loadImage(
  //   "/assets/itemImages/" + image || "empty.png"
  // );
  const thumbnail = context.items[image || "empty.png"];

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
  if (amount > 99) {
    ctx.fillText(amount, 45, 83);
  } else if (amount > 9) {
    ctx.fillText(amount, 60, 85);
  } else {
    ctx.fillText(amount, 75, 83);
  }
}

function loadImage(url) {
  return new Promise((resolve, reject) => {
    let imageObj = new Image();
    imageObj.onload = () => resolve(imageObj);
    imageObj.onerror = (e) => {
      // If the image fails to load, use the default image
      reject("Failed to load image");
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
