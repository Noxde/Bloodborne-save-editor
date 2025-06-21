import "./character.css";
import { useContext, useEffect, useState } from "react";
import { SaveContext } from "../../context/context";
import Stat from "../../components/Stat";
import Select from "../../components/Select";
import { invoke, dialog } from "@tauri-apps/api";
import { ImagesContext } from "../../context/imagesContext";
import Playtime from "./Playtime";
import { represent } from "../../utils/playtime";

function Character() {
  const { save, setSave } = useContext(SaveContext);
  const [username, setUsername] = useState(save.username.string);
  const [editedStats, setEditedStats] = useState(
    JSON.parse(JSON.stringify(save))
  );
  const [isz, setIsz] = useState([]);
  const [editedPlaytime, setEditedPlaytime] = useState(save.playtime);

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
  const ng = ["NG0", "NG+1", "NG+2", "NG+3", "NG+4", "NG+5", "NG+6", "NG+7"];
  const { images } = useContext(ImagesContext);

  useEffect(() => {
    invoke("get_isz").then((d) => setIsz(d));
  }, []);

  return (
    <div
      style={{
        gridColumn: "2/4",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        background: `url(${images.backgrounds["statsBg.png"].src})`,
        backgroundSize: "cover",
      }}
    >
      <div>
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            width: "100%",
            borderBottom: "1px solid rgb(107, 95, 73)",
          }}
        >
          <label htmlFor="username">Name:</label>
          <input
            id="username"
            type="text"
            autoComplete="off"
            spellCheck="false"
            maxLength={16}
            value={username}
            onChange={(e) => {
              setUsername(e.target.value);
            }}
            style={{
              width: "17ch",
              padding: "5px",
              textAlign: "right",
              background: "none",
            }}
          />
        </div>
        <div id="currency" style={{ padding: "0 0px" }}>
          <Stat
            editedStats={editedStats}
            setEditedStats={setEditedStats}
            stat={save.stats.find((x) => x.name === "Echoes")}
            width={"100%"}
          />

          <Stat
            editedStats={editedStats}
            setEditedStats={setEditedStats}
            stat={save.stats.find((x) => x.name === "Insight")}
            width={"100%"}
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
            <Select
              name={"Ng"}
              options={ng}
              setEditedStats={setEditedStats}
              editedStats={editedStats}
            />
          </div>
          {/* Appearance */}
          <div
            style={{
              fontSize: "25px",
              marginTop: "5px",
              display: "flex",
              justifyContent: "space-between",
            }}
          >
            <button
              className="buttonBg"
              style={{
                padding: "0 15px",
                fontSize: "inherit",
                backgroundSize: "100% 100%",
              }}
              onClick={async () => {
                try {
                  const path = await dialog.save({
                    title: "Save face file",
                  });

                  if (path) {
                    const success = await invoke("export_appearance", {
                      path,
                    });

                    await dialog.message(success);
                  }
                } catch (error) {
                  console.error(error);
                }
              }}
            >
              Export face
            </button>
            <button
              className="buttonBg"
              style={{
                padding: "0 15px",
                fontSize: "inherit",
                backgroundSize: "100% 100%",
              }}
              onClick={async () => {
                try {
                  const path = await dialog.open({
                    title: "Select a face file",
                  });

                  if (path) {
                    const success = await invoke("import_appearance", {
                      path,
                    });

                    await dialog.message(success);
                  }
                } catch (error) {
                  console.error(error);
                  await dialog.message(error, {
                    type: "error",
                  });
                }
              }}
            >
              Import face
            </button>
          </div>
          {/* Isz glitch */}
          <div
            style={{
              fontSize: "25px",
              marginTop: "5px",
              display: "flex",
              justifyContent: "space-between",
            }}
          >
            <span>
              Isz status:{" "}
              {isz.map((x) => x.toString(16).toUpperCase()).join(" ")}
            </span>
            <button
              style={{
                width: "174px",
                fontSize: "25px",
                padding: "0 15px",
                backgroundSize: "100% 100%",
              }}
              className="buttonBg"
              onClick={async () => {
                await invoke("fix_isz");
                setIsz(await invoke("get_isz"));
              }}
            >
              Fix isz
            </button>
          </div>
          <Playtime ms={editedPlaytime} setMs={setEditedPlaytime} />
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
                setUsername(save.username.string);
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

                  if (
                    username.length > 0 &&
                    username !== save.username.string
                  ) {
                    await invoke("set_username", {
                      newUsername: username,
                    });
                    editedStats.username.string = username;
                  } else {
                    setUsername(save.username.string);
                  }
                  await invoke("set_playtime", {
                    newPlaytime: represent(editedPlaytime),
                  });

                  editedStats.playtime = editedPlaytime;

                  await dialog.message("Confirmed changes");

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
