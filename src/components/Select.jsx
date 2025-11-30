import { useState, useContext, useEffect } from "react";
import { SaveContext } from "../context/context";

function Select({ options, name, setEditedStats, editedStats }) {
  const { save } = useContext(SaveContext);
  const [value, setValue] = useState(
    save.stats.find((y) => y.name === name)?.value
  );

  function handleChange(e) {
    const { target } = e;
    setValue(target.value);
    editedStats.stats.find((x) => x.name === name).value = target.value;

    setEditedStats(editedStats);
  }

  useEffect(() => {
    setValue(editedStats.stats.find((x) => x.name === name).value);
  }, [editedStats]);

  return (
    <div className="select-wrapper">
      <select value={value} name={name} onChange={handleChange}>
        {options.map((x, i) => {
          const label = x.label ?? x;
          const value = x.value ?? i;

          return (
            <option key={i} value={value}>
              {label}
            </option>
          );
        })}
      </select>
    </div>
  );
}

export default Select;
