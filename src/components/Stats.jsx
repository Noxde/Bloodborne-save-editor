import { useContext, useState } from "react";
import { SaveContext } from "../context/context";
import Stat from "./Stat";
import { invoke } from "@tauri-apps/api";

function Stats() {
  const { save, setSave } = useContext(SaveContext);
  const [editedStats, setEditedStats] = useState(
    JSON.parse(JSON.stringify(save))
  );

  return (
    <div
      style={{
        gridColumn: "2/4",
        display: "grid",
        gridTemplateColumns: "1fr 1fr",
        gridTemplateRows: "repeat(5, 50px)",
        gap: "0.8rem",
        alignItems: "start",
        justifyItems: "start",
        padding: "2rem",
        fontSize: "1.5rem",
        background: `url(${process.env.PUBLIC_URL + "/assets/statsBg.png"})`,
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
            try {
              stats.forEach(async ({ rel_offset, length, times, value }) => {
                await invoke("edit_stat", {
                  relOffset: rel_offset,
                  length,
                  times,
                  value: parseInt(value),
                });
              });

              setSave(JSON.parse(JSON.stringify(editedStats)));
            } catch (error) {
              console.error(error);
            }
          }}
        >
          Confirm
        </button>
      </div>
    </div>
  );
}

export default Stats;
