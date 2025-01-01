import { useContext, useState } from "react";
import { invoke } from "@tauri-apps/api";
import {
  getGemPath,
  getRunePath,
  isCursed,
  getUnique,
} from "../utils/drawCanvas";
import { SaveContext } from "../context/context";
import { ItemsContext } from "../context/itemsContext";
import SelectSearch from "./SelectSearch";
import useDraw from "../utils/useDraw";

function EditUpgrade({ setSelected, selected, setEditScreen, selectedRef }) {
  const { gemEffects, runeEffects } = useContext(ItemsContext);
  const drawCanvas = useDraw();

  const [edited, setEdited] = useState(JSON.parse(JSON.stringify(selected)));
  const {
    shape,
    effects,
    upgrade_type,
    info: { effect, rating, level, name, note },
    source,
  } = edited;
  const { setSave } = useContext(SaveContext);

  async function handleConfirm() {
    try {
      if (shape !== selected.shape)
        await invoke("edit_shape", {
          upgradeId: selected.id,
          upgradeType: upgrade_type,
          newShape: shape,
        });

      effects.forEach(async (x, i) => {
        const [id] = x;
        if (x === selected.effects[i][0]) return;

        await invoke("edit_effect", {
          upgradeId: edited.id,
          upgradeType: upgrade_type,
          newEffectId: parseInt(id),
          index: i,
        });
      });

      setSave((prev) => {
        const upgrade = prev.upgrades[selected.upgrade_type].find(
          (x) => x.id === edited.id
        );
        upgrade.shape = shape;
        upgrade.info.effect = effect;
        upgrade.info.rating = rating;
        upgrade.info.level = level;

        upgrade.effects = [...effects];
        upgrade.note = note;
        if (!isCursed(effects)) {
          edited.info.name = name.replace("Cursed ", "");
        } else {
          if (!edited.info.name.includes("Cursed")) {
            edited.info.name = `Cursed ${name}`;
          }
        }
        return prev;
      });
      setSelected(edited);

      const ctx = selectedRef.current.getContext("2d");
      selectedRef.current.dataset.item = JSON.stringify(edited);

      // setSelected(newJson);
      ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
      await drawCanvas(ctx, edited);

      setEditScreen(false);
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
          padding: "0 5rem",
          height: "95%",
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
            <span>Editing:</span>
            <div
              style={{
                position: "relative",
                display: "flex",
                justifyContent: "center",
                alignItems: "center",
                width: "375px",
                height: "375px",
                background: `url(${process.env.PUBLIC_URL}/assets/gems/gem_bg.png)`,
                backgroundSize: "contain",
                borderRadius: "5px",
              }}
            >
              <img
                width={"373px"}
                style={{ borderRadius: "10px" }}
                src={
                  upgrade_type === "Gem"
                    ? getGemPath(
                        effects,
                        shape,
                        level,
                        getUnique(effects[0][0], shape, source)
                      )
                    : getRunePath(name, shape, rating)
                }
                alt=""
              />
            </div>
          </div>
          <div>
            <button
              onClick={() => setEditScreen(false)}
              style={{ marginRight: "50px" }}
            >
              Cancel
            </button>
            <button onClick={handleConfirm}>Confirm</button>
          </div>
        </div>
        {/* List and input */}
        <div
          style={{
            position: "relative",
            marginTop: "20px",
            height: "95%",
            width: "371px",
            borderRadius: "0px",
            background: `url(/assets/${
              upgrade_type === "Gem"
                ? "gems/gem_effects_bg"
                : "runes/rune_effects_bg"
            }.png)`,
            backgroundRepeat: "no-repeat",
          }}
        >
          <div
            className="upgrade-info"
            style={{
              fontSize: "18px",
              userSelect: "none",
              color: "#b8b7ad",
            }}
          >
            {upgrade_type === "Gem" ? (
              <>
                <span
                  style={{
                    position: "absolute",
                    right: 15,
                    top: 20,
                  }}
                >
                  {rating}
                </span>
                <SelectSearch
                  style={{
                    position: "absolute",
                    right: 15,
                    top: 60,
                    textAlign: "right",
                  }}
                  selected={shape}
                  readOnly={true}
                  options={[
                    { label: "Radial", value: 1 },
                    { label: "Triangle", value: 2 },
                    { label: "Waning", value: 4 },
                    { label: "Circle", value: 8 },
                    { label: "Droplet", value: 64 },
                  ]}
                  onChange={(e) => {
                    const { label } = e;
                    setEdited((prevEdited) => {
                      const newEdited = { ...prevEdited };
                      newEdited.shape = label;

                      return newEdited;
                    });
                  }}
                />
              </>
            ) : (
              <SelectSearch
                style={{
                  position: "absolute",
                  right: 15,
                  top: 20,
                  textAlign: "right",
                }}
                selected={shape}
                readOnly={true}
                options={[
                  { label: "-", value: 1 },
                  { label: "Oath", value: 2 },
                ]}
                onChange={(e) => {
                  const { label } = e;
                  setEdited((prevEdited) => {
                    const newEdited = { ...prevEdited };
                    newEdited.shape = label;

                    return newEdited;
                  });
                }}
              />
            )}
          </div>
          <div
            className="effects"
            style={{
              fontSize: "18px",
              userSelect: "none",
              color: "#b8b7ad",
              position: "absolute",
              top: 180,
              left: 40,
              width: "calc(100% - 40px)",
            }}
          >
            {selected.effects.map(([id, name], i) => (
              <div className="effect">
                <SelectSearch
                  defaultValue={"No Effect"}
                  onChange={(e) => {
                    const { name, level, rating, value, label, note } = e;
                    setEdited((prevEdited) => {
                      const newEdited = { ...prevEdited };
                      if (i === 0) {
                        newEdited.info = {
                          ...newEdited.info,
                          effect: label,
                          note,
                          name,
                          level,
                          rating,
                        };
                      }
                      newEdited.effects[i] = [parseInt(value), label];

                      console.log(e, newEdited, i);

                      return newEdited;
                    });
                  }}
                  selected={name}
                  options={upgrade_type === "Gem" ? gemEffects : runeEffects}
                />
                <div
                  className="line"
                  style={{
                    position: "absolute",
                    left: -29,
                    width: "350px",
                    height: "1px",
                    background: `url(${process.env.PUBLIC_URL}/assets/line.png)`,
                  }}
                ></div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

export default EditUpgrade;

/**
 * Tear Stone gem:
 * Source: F1 49 02 80
 * Primary Effect: F0 F6 2F 00
 * Shape: Droplet
 *
 * Gold Pendand gem:
 * Source: F2 49 02 80
 * Primary Effect: DF CF 2F 00
 * Shape: Radial
 *
 * Brooch gem:
 * Source: F0 49 02 80
 * Effects: BC B3 2F 00 | 54 77 2E 00
 * Shape: Droplet
 */
