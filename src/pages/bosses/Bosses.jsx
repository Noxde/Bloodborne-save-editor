import { useContext, useEffect, useRef } from "react";
import { SaveContext } from "../../context/context";
import { invoke } from "@tauri-apps/api/core";
import { ImagesContext } from "../../context/imagesContext";
import Boss from "./Boss";

function Bosses() {
  const { save, setSave } = useContext(SaveContext);
  const { bosses } = save;
  const scrollDiv = useRef(null);

  const { images } = useContext(ImagesContext);

  function isDead(boss) {
    const flag = boss.flags.find((f) => f._comment !== "mp");
    return (flag.dead_value & flag.current_value & 0xff) !== 0;
  }

  async function handleChange(e, alive, i) {
    for (const x of e) {
      if (x._comment === "mp") {
        // Bosses sharing the same mp flag (same area) must all be dead to disable it
        const sharing = bosses.filter((b) =>
          b.flags.some(
            (f) =>
              f._comment === "mp" &&
              f.rel_offset === x.rel_offset &&
              f.dead_value === x.dead_value,
          ),
        );
        if (!alive && !sharing.every(isDead)) continue;

        // Mask instead of set, other flags share the same byte
        await invoke(alive ? "apply_mask_and" : "apply_mask_or", {
          offset: x.rel_offset,
          mask: alive ? x.alive_value : x.dead_value,
        });
        x.current_value = alive
          ? x.current_value & x.alive_value
          : x.current_value | x.dead_value;
      } else {
        await invoke("set_flag", {
          offset: x.rel_offset,
          newValue: x.current_value,
        });
      }
    }

    bosses[i].flags = e;
    setSave((prev) => {
      prev.bosses = bosses;
      return prev;
    });
  }

  useEffect(() => {
    if (scrollDiv?.current) {
      scrollDiv.current.scroll(0, -999);
    }
  }, [scrollDiv]); // Correct scroll

  return (
    <div
      ref={scrollDiv}
      style={{
        gridColumn: "2/4",
        display: "grid",
        gridTemplateColumns: "1fr 1fr",
        gridAutoRows: "50px",
        gap: "0.8rem 5rem",
        placeItems: "center",
        padding: "1.5rem",
        fontSize: "1.5rem",
        background: `url(${images.backgrounds["statsBg.png"].src})`,
        backgroundSize: "cover",
        overflowY: "auto",
      }}
    >
      {bosses.map((x, i) => {
        return (
          <Boss boss={x} onChange={async (e, alive) => await handleChange(e, alive, i)} />
        );
      })}
    </div>
  );
}

export default Bosses;
