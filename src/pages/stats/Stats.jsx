import { useContext, useState } from "react";
import { SaveContext } from "../../context/context";
import Stat from "../../components/Stat";
import { dialog, invoke } from "@tauri-apps/api";
import { ImagesContext } from "../../context/imagesContext";

function Stats() {
  const { save, setSave } = useContext(SaveContext);
  const [editedStats, setEditedStats] = useState(
    JSON.parse(JSON.stringify(save))
  );
  const { images } = useContext(ImagesContext);

  return (
    <div
      style={{
        alignContent: "center",
        gridColumn: "2/4",
        display: "grid",
        gridTemplateColumns: "1fr 1fr",
        gridTemplateRows: "repeat(5, 50px)",
        gap: "0.8rem 0",
        alignItems: "center",
        justifyItems: "center",
        placeItems: "center",
        // padding: "1.5rem",
        fontSize: "1.5rem",
        background: `url(${images.backgrounds["statsBg.png"].src})`,
        backgroundSize: "cover",
      }}
    >
      {editedStats.stats
        .filter(
          (x) =>
            x.name !== "Echoes" &&
            x.name !== "Insight" &&
            x.name !== "Voice" &&
            x.name !== "Gender" &&
            x.name !== "Ng" &&
            x.name !== "Origin"
        )
        .map((x, i) => (
          <Stat
            editedStats={editedStats}
            setEditedStats={setEditedStats}
            key={i}
            stat={x}
          />
        ))}
      <div
        style={{
          display: "flex",
          justifyContent: "space-evenly",
          justifySelf: "center",
          width: "100%",
        }}
      >
        <button
          className="btn-underline"
          style={{
            position: "relative",
            padding: "0 1rem",
          }}
          onClick={() => {
            setEditedStats(JSON.parse(JSON.stringify(save)));
          }}
        >
          Reset
        </button>
        <button
          className="btn-underline"
          style={{
            position: "relative",
            padding: "0 1rem",
          }}
          onClick={async () => {
            const { stats } = editedStats;
            stats.forEach(async ({ rel_offset, length, times, value }) => {
              try {
                await invoke("edit_stat", {
                  relOffset: rel_offset,
                  length,
                  times,
                  value: parseInt(value),
                });
              } catch (error) {
                console.error(error);
              }
            });
            setSave(JSON.parse(JSON.stringify(editedStats)));
            await dialog.message("Confirmed changes");
          }}
        >
          Confirm
        </button>
      </div>
    </div>
  );
}

export default Stats;
