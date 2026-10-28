import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { useEffect, useRef, useState } from "react";

function Flag({ label, offset, values, info, isMask = false, onClick }) {
  const tipRef = useRef();
  const [top, setTop] = useState(0);
  const [showTip, setShowTip] = useState(false);
  const [tooltipBelow, setTooltipBelow] = useState(false);

  useEffect(() => {
    if (tipRef.current) {
      setTop(tipRef.current.getBoundingClientRect().height);
    }
  }, []);

  async function setFlag() {
    for (let i = 0; i < values.length; i++) {
      await invoke("set_flag", {
        offset: offset + i,
        newValue: values[i],
      });
    }
    await message("Flag applied");
  }

  async function applyMask() {
    await invoke("apply_mask_and", {
      offset: offset,
      mask: values[0],
    });
  }

  function showTooltip() {
    const tooltip = tipRef.current;
    const row = tooltip?.parentElement;
    const list = row?.closest(".flags-list");

    if (tooltip && row && list) {
      const rowTop = row.getBoundingClientRect().top;
      const listTop = list.getBoundingClientRect().top;
      setTooltipBelow(
        rowTop - tooltip.getBoundingClientRect().height - 5 < listTop,
      );
    }

    setShowTip(true);
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
          top: tooltipBelow ? "calc(70% + 5px)" : `-${top + 5}px`,
          zIndex: 999,
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
          onMouseEnter={showTooltip}
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
        onClick={
          typeof onClick === "function"
            ? async () => {
                onClick();
                await message("Flags applied");
              }
            : isMask
              ? applyMask
              : setFlag
        }
      >
        Apply
      </button>
    </div>
  );
}

export default Flag;
