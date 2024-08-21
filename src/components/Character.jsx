import "./character.css";
import { useContext, useState } from "react";
import { SaveContext } from "../context/context";
import Stat from "./Stat";
import Select from "./Select";
import { invoke } from "@tauri-apps/api";

function Character() {
  const { save, setSave } = useContext(SaveContext);
  const [editedStats, setEditedStats] = useState(
    JSON.parse(JSON.stringify(save))
  );
  const voice = ["Young Voice", "Mature Voice", "Aged Voice"];
  const gender = ["Female", "Male"];
  const origins = [
    "Milquetoast",
    "Lone Survivor",
    "Troubled Childhood",
    "Violent Past",
    "Professional",
    "Military Veteran",
    "Noble Scion",
    "Cruel Fate",
    "Waste of Skin",
  ];

  return (
    <div
      style={{
        gridColumn: "2/4",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        background: `url(${process.env.PUBLIC_URL + "/assets/statsBg.png"})`,
        backgroundSize: "cover",
      }}
    >
      <div>
        <div id="currency" style={{ padding: "0 20px" }}>
          <Stat
            editedStats={editedStats}
            setEditedStats={setEditedStats}
            stat={save.stats.find((x) => x.name === "Echoes")}
          />

          <Stat
            editedStats={editedStats}
            setEditedStats={setEditedStats}
            stat={save.stats.find((x) => x.name === "Insight")}
          />
        </div>
        <div id="characterData">
          <div
            style={{
              display: "flex",
              justifyContent: "space-evenly",
              marginTop: "5px",
            }}
          >
            <Select
              name={"Gender"}
              options={gender}
              setEditedStats={setEditedStats}
              editedStats={editedStats}
            />
            <Select
              name={"Origin"}
              options={origins}
              setEditedStats={setEditedStats}
              editedStats={editedStats}
            />
            <Select
              name={"Voice"}
              options={voice}
              setEditedStats={setEditedStats}
              editedStats={editedStats}
            />
          </div>
          <div
            style={{
              display: "flex",
              marginTop: "5rem",
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
                  stats.forEach(
                    async ({ rel_offset, length, times, value }) => {
                      await invoke("edit_stat", {
                        relOffset: rel_offset,
                        length,
                        times,
                        value: parseInt(value),
                      });
                    }
                  );

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
      </div>
    </div>
  );
}

export default Character;
