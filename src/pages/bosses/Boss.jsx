function Boss({ boss, onChange }) {
  let { name, flags } = boss;
  let [defeatedFlag] = flags;

  function handleChange({ target }) {
    const option = JSON.parse(target.value);

    if (option === 1) {
      flags.forEach((f) => {
        f.current_value = f.alive_value;
      });
    } else {
      flags.forEach((f) => {
        f.current_value = f.dead_value;
      });
    }
    // boss.value = value;
    if (typeof onChange === "function") {
      onChange(flags);
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
          <option value={1}>Alive</option>
          <option
            selected={
              (defeatedFlag.dead_value & defeatedFlag.current_value & 0xff) !==
              0
            }
            value={2}
          >
            Dead
          </option>
        </select>
      </div>
    </div>
  );
}

export default Boss;
