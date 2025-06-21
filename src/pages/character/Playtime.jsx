import "./playtime.css";
import { useEffect, useState } from "react";
import { interpret, toMs } from "../../utils/playtime";

function Playtime({ ms, setMs }) {
  const [time, setTime] = useState(interpret(ms));

  useEffect(() => {
    setMs(toMs(time));
  }, [time]);

  function handleChange(e) {
    const { name, value } = e.target;
    if (value < 0) return;

    setTime((prev) => ({ ...prev, [name]: parseInt(value) }));
  }

  function handleInput(e) {
    const { value } = e.target;

    if (value.length === 1 || value.startsWith("00")) {
      e.target.value = `0${+e.target.value}`;
    } else {
      e.target.value = +e.target.value;
    }
  }

  return (
    <div
      id="playtime"
      style={{
        fontSize: "25px",
        marginTop: "5px",
        display: "flex",
        justifyContent: "space-between",
      }}
    >
      <span>Playtime:</span>
      <div
        style={{
          width: "174px",
          fontSize: "25px",
          display: "flex",
          justifyContent: "space-evenly",
          background: "#00000081",
        }}
      >
        <input
          value={time["hours"]}
          onChange={handleChange}
          onInput={handleInput}
          type="number"
          name="hours"
        />
        <span>:</span>
        <input
          value={time["minutes"]}
          onChange={handleChange}
          onInput={handleInput}
          type="number"
          name="minutes"
        />
        <span>:</span>
        <input
          value={time["seconds"]}
          onChange={handleChange}
          onInput={handleInput}
          type="number"
          name="seconds"
        />
      </div>
    </div>
  );
}

export default Playtime;
