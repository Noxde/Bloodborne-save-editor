import { invoke } from "@tauri-apps/api/core";

function Teleport({ setSave, setEditedCoordinates }) {
  async function handleChange(e) {
    const { target } = e;
    const parsed = JSON.parse(target.value);
    const { x, y, z, mapId } = parsed;

    await invoke("teleport", parsed);

    setEditedCoordinates({ x, y, z });

    setSave((prev) => {
      return JSON.parse(
        JSON.stringify({
          ...prev,
          position: {
            coordinates: {
              x,
              y,
              z,
            },
            loaded_map: mapId,
          },
        }),
      );
    });
  }

  return (
    <div
      style={{
        fontSize: "25px",
        marginTop: "5px",
        display: "flex",
        justifyContent: "space-between",
      }}
    >
      <span>Teleport:</span>
      <div
        style={{
          fontSize: "25px",
          display: "flex",
          gap: "5px",
        }}
      >
        <select name="location" id="location-select" onChange={handleChange}>
          <option selected hidden disabled>
            Select a location
          </option>
          <option value='{"x":-8,"y":-6,"z":-18,"mapId":[21,0]}'>
            Hunter's Dream
          </option>
          <optgroup label="Yharnam Headstone">
            <option value='{"x":-199.74,"y":-50.759,"z":179.42,"mapId":[24,1]}'>
              1st Floor Sickroom
            </option>
            <option value='{"x":-193.4,"y":-28.646,"z":68.5,"mapId":[24,1]}'>
              Central Yharnam
            </option>
            <option value='{"x":-124.488,"y":-27.021,"z":64.673,"mapId":[24,1]}'>
              Great Bridge
            </option>
            <option value='{"x":-33.811,"y":-40.722,"z":87.303,"mapId":[24,1]}'>
              Tomb of Oedon
            </option>
            <option value='{"x":16.775,"y":-9.511,"z":103.27,"mapId":[24,0]}'>
              Cathedral Ward
            </option>
            <option value='{"x":67.808,"y":35.713,"z":339.689,"mapId":[24,0]}'>
              Grand Cathedral Ward
            </option>
            <option value='{"x":-24.643,"y":40.621,"z":250.57,"mapId":[24,2]}'>
              Upper Cathedral Ward
            </option>
            <option value='{"x":45.335,"y":51.403,"z":300.35,"mapId":[24,2]}'>
              Lumenflower Gardens
            </option>
            <option value='{"x":114.86,"y":4.443,"z":425.02,"mapId":[24,2]}'>
              Altar of Despair
            </option>
            <option value='{"x":126.4,"y":-65.214,"z":36,"mapId":[23,0]}'>
              Old Yharnam
            </option>
            <option value='{"x":-139.979,"y":-126.664,"z":57.359,"mapId":[23,0]}'>
              Church of the Good Chalice
            </option>
            <option value='{"x":111.86,"y":-120.783,"z":-65.249,"mapId":[23,0]}'>
              Graveyard of the Darkbeast
            </option>
          </optgroup>
          <optgroup label="Frontier Headstone">
            <option value='{"x":-172,"y":-22,"z":485.5,"mapId":[22,0]}'>
              Hemwick Charnel Lane
            </option>
            <option value='{"x":-336.3,"y":2.4,"z":733,"mapId":[22,0]}'>
              Witch's Abode
            </option>
            <option value='{"x":-190,"y":-76.3,"z":252,"mapId":[27,0]}'>
              Forbidden Woods
            </option>
            <option value='{"x":-335,"y":-186.5,"z":479,"mapId":[27,0]}'>
              Forbidden Grave
            </option>
            <option value='{"x":-400.4,"y":-180.8,"z":414.6,"mapId":[32,0]}'>
              Byrgenwerth
            </option>
          </optgroup>
          <optgroup label="Unseen Headstone">
            <option value='{"x":257.4,"y":-51.4,"z":70,"mapId":[28,0]}'>
              Yahar'gul, Unseen Village
            </option>
            <option value='{"x":260.4,"y":-88,"z":-55.6,"mapId":[28,0]}'>
              Yahar'gul Chapel
            </option>
            <option value='{"x":418.8,"y":-123.6,"z":-253.4,"mapId":[28,0]}'>
              Advent Plaza
            </option>
            <option value='{"x":219.6,"y":-97.6,"z":-78.8,"mapId":[28,0]}'>
              Hypogean Gaol
            </option>
            <option value='{"x":-4.5,"y":33.8,"z":-187.9,"mapId":[25,0]}'>
              Forsaken Castle Cainhurst
            </option>
            <option value='{"x":47.8,"y":111.8,"z":-350.4,"mapId":[25,0]}'>
              Logarius' Seat
            </option>
            <option value='{"x":122.4,"y":129,"z":-455,"mapId":[25,0]}'>
              Vileblood Queen's Chamber
            </option>
            <option value='{"x":129.8,"y":-19.9,"z":140.8,"mapId":[21,1]}'>
              Abandoned Old Workshop
            </option>
          </optgroup>
          <optgroup label="Nightmare Headstone">
            <option value='{"x":-472.37,"y":-185.25,"z":594.9,"mapId":[32,0]}'>
              Lecture Building
            </option>
            <option value='{"x":-444.22,"y":-177.25,"z":514.19,"mapId":[32,0]}'>
              Lecture Building 2nd Floor
            </option>
            <option value='{"x":0.35,"y":1500,"z":0,"mapId":[33,0]}'>
              Nightmare Frontier
            </option>
            <option value='{"x":-104.65,"y":1462.28,"z":-42.65,"mapId":[33,0]}'>
              Nightmare of Mensis
            </option>
            <option value='{"x":84.58,"y":986.7,"z":-0.37,"mapId":[26,0]}'>
              Mergo's Loft: Base
            </option>
            <option value='{"x":136.69,"y":1061.26,"z":-14.86,"mapId":[26,0]}'>
              Mergo's Loft: Middle
            </option>
            <option value='{"x":140.72,"y":1124.3,"z":-37.98,"mapId":[26,0]}'>
              Wet Nurse's Lunarium
            </option>
          </optgroup>
          <optgroup label="Hunter's Nightmare Headstone">
            <option value='{"x":-481.68,"y":1490.49,"z":-497.73,"mapId":[34,0]}'>
              Hunter's Nightmare
            </option>
            <option value='{"x":-434.08,"y":1503.18,"z":-594.52,"mapId":[34,0]}'>
              Nightmare Church
            </option>
            <option value='{"x":-433.09,"y":1535.71,"z":-261.57,"mapId":[34,0]}'>
              Nightmare Grand Cathedral
            </option>
            <option value='{"x":-406.81,"y":1503.79,"z":-743,"mapId":[34,0]}'>
              Underground Corpse Pile
            </option>
            <option value='{"x":-318.67,"y":1553.02,"z":-824.22,"mapId":[35,0]}'>
              Research Hall
            </option>
            <option value='{"x":-432.15,"y":1593,"z":-824.37,"mapId":[35,0]}'>
              Lumenwood Garden
            </option>
            <option value='{"x":-454.88,"y":1595.57,"z":-824.44,"mapId":[35,0]}'>
              Astral Clocktower
            </option>
            <option value='{"x":-619.2,"y":1594.3,"z":-817.2,"mapId":[36,0]}'>
              Fishing Hamlet
            </option>
            <option value='{"x":-645.2,"y":1614.66,"z":-867.2,"mapId":[36,0]}'>
              Lighthouse Hut
            </option>
            <option value='{"x":-695.2,"y":1577.27,"z":-943.2,"mapId":[36,0]}'>
              Coast
            </option>
          </optgroup>
        </select>
      </div>
    </div>
  );
}

export default Teleport;
