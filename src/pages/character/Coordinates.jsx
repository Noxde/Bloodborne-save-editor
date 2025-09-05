function Coordinates({ coordinates: { x, y, z }, setCoordinates }) {
  return (
    <div
      style={{
        fontSize: "25px",
        marginTop: "5px",
        display: "flex",
        justifyContent: "space-between",
      }}
    >
      <span>Coordinates:</span>
      <div
        style={{
          fontSize: "25px",
          display: "flex",
          gap: "5px",
        }}
      >
        X:
        <input
          value={x}
          onChange={(e) =>
            setCoordinates((prev) => ({ ...prev, x: +e.target.value }))
          }
          style={{
            textAlign: "center",
            width: "85px",
          }}
          type="number"
        />
        Y:
        <input
          value={y}
          onChange={(e) =>
            setCoordinates((prev) => ({ ...prev, y: +e.target.value }))
          }
          style={{
            textAlign: "center",
            width: "85px",
          }}
          type="number"
        />
        Z:
        <input
          value={z}
          onChange={(e) =>
            setCoordinates((prev) => ({ ...prev, z: +e.target.value }))
          }
          style={{
            textAlign: "center",
            width: "85px",
          }}
          type="number"
        />
      </div>
    </div>
  );
}

export default Coordinates;
