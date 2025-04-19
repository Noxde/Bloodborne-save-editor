import { useContext, useEffect, useRef } from "react";
import { SaveContext } from "../context/context";
import { invoke } from "@tauri-apps/api";
import { ImagesContext } from "../context/imagesContext";
import Boss from "./Boss";

function Bosses() {
  const { save, setSave } = useContext(SaveContext);
  const { bosses } = save;
  const scrollDiv = useRef(null);

  const { images } = useContext(ImagesContext);

  async function handleChange(e, i) {
    await invoke("set_flag", {
      offset: e.rel_offset,
      newValue: e.value,
    });

    bosses[i] = e;
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
        alignContent: "center",
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
          <Boss boss={x} onChange={async (e) => await handleChange(e, i)} />
        );
      })}
    </div>
  );
}

export default Bosses;
