function Boss({ boss, onChange }) {
  let { name, flags } = boss;
  const defeatedFlag = flags.find((f) => f._comment !== "mp");

  function handleChange({ target }) {
    const alive = JSON.parse(target.value) === 1;

    // Multiplayer flags are masks, they're applied in Bosses
    flags.forEach((f) => {
      if (f._comment !== "mp") {
        f.current_value = alive ? f.alive_value : f.dead_value;
      }
    });

    if (typeof onChange === "function") {
      onChange(flags, alive);
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
        <select
          name="bossStatus"
          id="bossStatus"
          onChange={handleChange}
          defaultValue={
            (defeatedFlag.dead_value & defeatedFlag.current_value & 0xff) !== 0
              ? 2
              : 1
          }
        >
          <option value={1}>Alive</option>
          <option value={2}>Dead</option>
        </select>
      </div>
    </div>
  );
}

export default Boss;
