function Boss({ boss, onChange }) {
  let { name, dead_value, value } = boss;

  function handleChange({ target }) {
    const { mask, op } = JSON.parse(target.value);

    if (op === "AND") {
      value &= mask;
    } else {
      value |= mask;
    }
    boss.value = value;
    if (typeof onChange === "function") {
      onChange(boss);
    }
  }

  return (
    <div
      style={{
        width: "100%",
        display: "flex",
        justifyContent: "space-between",
        borderBottom: "1px solid #6b5f49",
      }}
    >
      <span>{name}:</span>
      <div className="select-wrapper">
        <select name="bossStatus" id="bossStatus" onChange={handleChange}>
          <option value={`{ "mask": ${~dead_value & 0xff}, "op": "AND" }`}>
            Alive
          </option>
          <option
            selected={(dead_value & value & 0xff) !== 0}
            value={`{ "mask": ${dead_value}, "op": "OR" }`}
          >
            Dead
          </option>
        </select>
      </div>
    </div>
  );
}

export default Boss;
