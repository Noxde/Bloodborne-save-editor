import Flag from "./Flag";

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
          overflowX: "hidden",
          height: "100%",
          width: "100%",
          paddingBottom: "2px",
        }}
      >
        <Flag label="Restore Maria's dialogue" offset={1083} values={[0, 8]} />
        <Flag label="Enable Doll's lullaby" offset={6689} values={[1, 8]} />
      </div>
    </div>
  );
}

export default Flags;
