import Flag from "./Flag";
import "./flags.css";

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
          justifyContent: "center",
          gap: "2rem",

          overflowY: "auto",
          overflowX: "visible",
          height: "100%",
          width: "100%",
          paddingBottom: "2px",
        }}
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
          info={
            "Enables the doll's lullaby from version 1.0, the doll needs to be sleeping to trigger when enabled."
          }
        />
        <Flag
          label="Enable Blood addled"
          offset={4127}
          values={[162]}
          info={"Enables pvp when looking for coop."}
        />
      </div>
    </div>
  );
}

export default Flags;
