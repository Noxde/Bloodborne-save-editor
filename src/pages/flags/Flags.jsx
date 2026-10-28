import Flag from "./Flag";
import "./flags.css";
import { invoke } from "@tauri-apps/api/core";

const mpFlags = [
  { offset: 1678, mask: 208 }, // CentralYharnam: xx1x xxxx
  { offset: 1677, mask: 63 }, // CathedralWard: 11xx xxxx
  { offset: 1679, mask: 247 }, // UpperCathedralWard
  { offset: 1679, mask: 251 }, // AltarOfDespair
  { offset: 1664, mask: 247 }, // OldYharnamChurch: xxxx 1xxx
  { offset: 1664, mask: 251 }, // OldYharnamDarkBeast: xxxx x1xx
  { offset: 1714, mask: 243 }, // ForbiddenWoods: xxxx 11xx
  { offset: 2277, mask: 127 }, // Byrgenwerth: 1xxx xxxx
  { offset: 1652, mask: 127 }, // Hemwick: 1xxx xxxx
  { offset: 1689, mask: 247 }, // Cainhurst: xxxx 1xxx
  { offset: 1727, mask: 127 }, // Yahargul: 1xxx xxxx
  { offset: 1702, mask: 127 }, // NightmareOfMensisMicolash: 1xxx xxxx
  { offset: 1702, mask: 191 }, // NightmareOfMensisMergo: x1xx xxxx
  { offset: 2289, mask: 247 }, // NightmareFrontier: xxxx 1xxx
  // DLC
  { offset: 2302, mask: 63 }, // HunterNightmare: 11xx xxxx
  { offset: 2315, mask: 254 }, // ResearchHall: xxxx xxx1
  { offset: 2316, mask: 127 }, // ClockTower: 1xxx xxxx
  { offset: 2327, mask: 247 }, // HunterNightmare: xxxx 1xxx
];

async function enableMp() {
  for (const flag of mpFlags) {
    await invoke("apply_mask_and", {
      offset: flag.offset,
      mask: flag.mask,
    });
  }
}

function Flags() {
  return (
    <div
      style={{
        gridColumn: "2/4",
        display: "grid",
        gridTemplateRows: "minmax(370px, 60vh)",
        gridTemplateColumns: "50%",
        // gap: "5rem",
        alignContent: "center",
        // alignItems: "center",
        justifyContent: "center",
        // background: `url(${images.backgrounds["statsBg.png"].src})`,
        backgroundSize: "cover",
        position: "relative",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          justifyContent: "flex-start",
          gap: "2rem",

          overflowY: "auto",
          overflowX: "visible",
          height: "100%",
          minHeight: 0,
          width: "100%",
          paddingBottom: "2px",
        }}
        className="flags-list"
      >
        <Flag
          label="Restore Maria's dialogue"
          offset={1083}
          values={[0, 8]}
          info={"Restores some dialogue before fighting Maria."}
        />
        <Flag
          label="Enable Doll's lullaby"
          offset={6689}
          values={[8, 1]}
          info={"Enables the doll's lullaby from version 1.0."}
        />
        <Flag
          label="Enable Blood addled"
          offset={4127}
          values={[162]}
          info={
            "Enables pvp when doing coop with players that have the Hunter rune equipped."
          }
        />
        <Flag
          label="Enable multiplayer even with bosses dead"
          onClick={async () => {
            await enableMp();
            console.log("Clicked");
          }}
          info={
            "Enables multiplayer on all maps except for cathedral ward, yahargul and dlc."
          }
        />
      </div>
    </div>
  );
}

export default Flags;
