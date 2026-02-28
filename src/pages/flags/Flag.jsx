import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";

function Flag({ label, offset, values, info }) {
  const tipRef = useRef();
  const [top, setTop] = useState(0);
  const [showTip, setShowTip] = useState(false);

  useEffect(() => {
    if (tipRef.current) {
      setTop(tipRef.current.getBoundingClientRect().height);
    }
  }, []);

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
        alignItems: "center",
        width: "100%",
        position: "relative",
      }}
    >
      {/* Tooltip info */}
      <div
        className="tooltip"
        style={{
          opacity: showTip ? 1 : 0,
          top: `-${top + 5}px`,
        }}
        ref={tipRef}
      >
        {info}
      </div>

      {/* Tooltip hover and label */}
      <div
        style={{
          position: "relative",
          paddingLeft: 25,
        }}
      >
        <div
          className="tooltip-hover"
          onMouseEnter={() => setShowTip(true)}
          onMouseLeave={() => setShowTip(false)}
        >
          ?
        </div>
        <label>{label}</label>
      </div>

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
