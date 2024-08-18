import { useState, useEffect } from "react";

function Stat({ stat, editedStats, setEditedStats }) {
  const [value, setValue] = useState(stat.value);

  useEffect(() => {
    setValue(stat.value);
  }, [stat]);

  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        borderBottom: "1px solid #6b5f49",
      }}
    >
      <img
        width={32}
        height={32}
        style={{ marginRight: "10px" }}
        src={
          process.env.PUBLIC_URL +
          `/assets/stats/${stat.name.toLowerCase()}.png`
        }
        alt=""
      />
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          width: "433px",
        }}
      >
        <label>{stat.name}: </label>
        <input
          type="number"
          style={{
            width: "8rem",
            textAlign: "right",
            background: "none",
          }}
          min={0}
          value={value}
          onChange={(e) => {
            const { value: newValue } = e.target;
            setValue(newValue);

            editedStats.stats.find((x) => x.name === stat.name).value =
              newValue;

            setEditedStats(editedStats);
          }}
        />
      </div>
    </div>
  );
}

export default Stat;
