import "./character.css";
import { useContext, useState } from "react";
import { SaveContext } from "../../context/context";
import Stat from "../../components/Stat";
import { invoke } from "@tauri-apps/api/core";
import { ImagesContext } from "../../context/imagesContext";
import Playtime from "./Playtime";
import { represent } from "../../utils/playtime";
import CharacterInfo from "./CharacterInfo";
import Appearance from "./Appearance";
import IszGlitch from "./IszGlitch";
import Coordinates from "./Coordinates";
import Teleport from "./Teleport";
import * as dialog from "@tauri-apps/plugin-dialog";

function Character() {
  const { save, setSave } = useContext(SaveContext);
  const [username, setUsername] = useState(save.username.string);
  const [editedStats, setEditedStats] = useState(
    JSON.parse(JSON.stringify(save)),
  );
  const [editedPlaytime, setEditedPlaytime] = useState(save.playtime);
  const [editedCoordinates, setEditedCoordinates] = useState(
    save.position.coordinates,
  );

  const { images } = useContext(ImagesContext);

  return (
    <div
      style={{
        gridColumn: "2/4",
        display: "grid",
        gridTemplateRows: "minmax(370px, 60vh) min-content",
        gap: "5rem",
        alignContent: "center",
        alignItems: "center",
        justifyContent: "center",
        background: `url(${images.backgrounds["statsBg.png"].src})`,
        backgroundSize: "cover",
        position: "relative",
      }}
    >
      <div
        style={{
          overflowY: "auto",
          overflowX: "hidden",
          height: "100%",
          paddingBottom: "2px",
        }}
      >
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
          <CharacterInfo
            editedStats={editedStats}
            setEditedStats={setEditedStats}
          />
          {/* Appearance */}
          <Appearance />
          {/* Isz glitch */}
          <IszGlitch />
          <Playtime ms={editedPlaytime} setMs={setEditedPlaytime} />
          <Coordinates
            coordinates={editedCoordinates}
            setCoordinates={setEditedCoordinates}
          />
          <Teleport
            setSave={setSave}
            setEditedCoordinates={setEditedCoordinates}
          />
        </div>
      </div>
      {/* Buttons */}
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
            setEditedPlaytime(save.playtime);
            setEditedCoordinates(save.position.coordinates);
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
              stats.forEach(async ({ rel_offset, length, times, value }) => {
                await invoke("edit_stat", {
                  relOffset: rel_offset,
                  length,
                  times,
                  value: parseInt(value),
                });
              });

              if (username.length > 0 && username !== save.username.string) {
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

              await invoke("edit_coordinates", {
                x: +editedCoordinates.x,
                y: +editedCoordinates.y,
                z: +editedCoordinates.z,
              });

              editedStats.position.coordinates = editedCoordinates;

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
  );
}

export default Character;
