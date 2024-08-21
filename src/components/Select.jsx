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
    <select value={value} name={name} onChange={handleChange}>
      {options.map((x, i) => (
        <option key={i} value={i}>
          {x}
        </option>
      ))}
    </select>
  );
}

export default Select;
