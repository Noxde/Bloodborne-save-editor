import { useEffect, useRef, useState } from "react";
import { FixedSizeList as List } from "react-window";
import { useFloating, flip, offset, autoUpdate } from "@floating-ui/react";

function SelectSearch({
  options,
  selected,
  readOnly = false,
  onChange,
  style,
  defaultValue,
}) {
  const [isOpen, setIsOpen] = useState(false);
  const [search, setSearch] = useState(
    selected === defaultValue ? "" : selected,
  );

  // 1. Initialize Floating UI
  const { refs, floatingStyles } = useFloating({
    open: isOpen,
    onOpenChange: setIsOpen,
    middleware: [
      offset(4), // Little gap between input and dropdown
      flip({ fallbackAxisSideDirection: "start" }), // Automatically opens up if bottom is blocked
    ],
    whileElementsMounted: autoUpdate, // Keeps positioning accurate if window resizes/scrolls
  });

  const dropdownRef = useRef();

  // Handle outside clicks safely
  useEffect(() => {
    function toggle(e) {
      if (dropdownRef.current && !dropdownRef.current.contains(e.target)) {
        setIsOpen(false);
      }
    }
    document.addEventListener("mousedown", toggle);
    return () => document.removeEventListener("mousedown", toggle);
  }, []);

  useEffect(() => {
    if (!search.trim() && !isOpen) {
      if (typeof onChange === "function") {
        onChange({
          label: "No Effect",
          rating: 95,
          level: 0,
          name: "",
          value: "4294967295",
        });
      }
      setSearch("");
    }
  }, [search, isOpen]);

  const filteredOptions = options.filter((x) =>
    !readOnly ? x.label.toLowerCase().includes(search.toLowerCase()) : true,
  );

  function Row({ index, style: rowStyle }) {
    return (
      <div
        style={{
          ...rowStyle,
          background: "black",
          borderBottom: "1px solid #444",
          fontSize: "16px",
          display: "flex",
          alignItems: "center",
          cursor: "pointer",
        }}
        onClick={() => {
          setSearch(filteredOptions[index].label);
          if (typeof onChange === "function") {
            onChange(filteredOptions[index]);
          }
          setIsOpen(false); // Close menu on select
        }}
      >
        {filteredOptions[index].label}
      </div>
    );
  }

  return (
    // Attach the outside click ref and base styles here
    <div ref={dropdownRef} style={{ ...style, position: "relative" }}>
      {/* 2. Attach reference ref to the trigger container */}
      <div ref={refs.setReference} style={{ width: "100%" }}>
        <input
          value={search}
          readOnly={readOnly}
          placeholder={defaultValue}
          onFocus={() => setIsOpen(true)}
          onChange={({ target: { value } }) => setSearch(value)}
          style={{
            width: "100%",
            background: "none",
            fontSize: "18px",
            textAlign: "inherit",
          }}
        />
      </div>

      {/* 3. Dropdown Menu */}
      {isOpen && (
        <div
          ref={refs.setFloating}
          style={{
            ...floatingStyles, // Floating UI injects left/top absolute coordinates dynamically
            width: dropdownRef.current
              ? dropdownRef.current.offsetWidth
              : "100%",
            zIndex: 9999,
            background: "black",
            border: "1px solid #444",
            boxShadow: "0px 4px 12px rgba(0,0,0,0.5)",
          }}
        >
          <List
            height={Math.min(210, filteredOptions.length * 35)} // Shrinks menu if search yields few results
            itemCount={filteredOptions.length}
            itemSize={35}
            width="100%"
            overscanCount={10}
          >
            {Row}
          </List>
        </div>
      )}
    </div>
  );
}

export default SelectSearch;
