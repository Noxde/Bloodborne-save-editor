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
      name.includes("DOWN"),
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
      lowerCaseEffect,
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

export { getType, getGemColor, getRunePath, getGemPath, isCursed, getUnique };
