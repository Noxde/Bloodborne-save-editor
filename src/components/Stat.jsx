import { useState, useEffect } from "react";

function Stat({ stat, editedStats, setEditedStats, width }) {
  const [value, setValue] = useState(stat.value);

  useEffect(() => {
    setValue(stat.value);
  }, [stat, editedStats]);

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
          width: width || "420px",
        }}
      >
        <label>{stat.name}: </label>
        <input
          type="number"
          style={{
            textAlign: "right",
            background: "none",
          }}
          min={0}
          value={value}
          onChange={(e) => {
            const { value: newValue } = e.target;
            if (newValue > 999999999) {
              setValue(999999999);

              editedStats.stats.find(
                (x) => x.name === stat.name
              ).value = 999999999;
            } else {
              setValue(newValue);

              editedStats.stats.find((x) => x.name === stat.name).value =
                newValue;
            }

            setEditedStats(editedStats);
          }}
        />
      </div>
    </div>
  );
}

export default Stat;
