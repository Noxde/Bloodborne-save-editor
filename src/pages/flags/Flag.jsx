import { invoke } from "@tauri-apps/api/core";

function Flag({ label, offset, values }) {
  async function setFlag() {
    console.log(values);
    for (let i = 0; i < values.length; i++) {
      await invoke("set_flag", {
        offset: offset + i,
        newValue: values[i],
      });
    }
  }

  return (
    <div
      style={{
        display: "flex",
        justifyContent: "space-between",
        width: "100%",
      }}
    >
      <label>{label}</label>
      <button
        style={{
          padding: "0rem 1rem",
          backgroundSize: "100% 100%",
        }}
        className="buttonBg"
        onClick={setFlag}
      >
        Apply
      </button>
    </div>
  );
}

export default Flag;
