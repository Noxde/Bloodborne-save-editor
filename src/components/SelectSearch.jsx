import { useEffect, useRef, useState } from "react";
import { FixedSizeList as List } from "react-window";

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
    selected === defaultValue ? "" : selected
  );
  const dropdownRef = useRef();

  useEffect(() => {
    function toggle(e) {
      if (!Array.from(dropdownRef.current.childNodes).includes(e.target)) {
        setIsOpen(false);
      }
    }

    document.addEventListener("click", toggle);
    return () => {
      document.removeEventListener("click", toggle);
    };
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
    !readOnly ? x.label.toLowerCase().includes(search.toLowerCase()) : true
  );

  const Row = ({ index, style }) => (
    <div
      style={{ ...style, background: "black", padding: "0 5px" }}
      onClick={() => {
        setSearch(filteredOptions[index].label);
        if (typeof onChange === "function") {
          onChange(filteredOptions[index]);
        }
      }}
    >
      {filteredOptions[index].label}
    </div>
  );

  return (
    <div ref={dropdownRef} style={style}>
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
      {isOpen && (
        <div style={{ position: "absolute", width: "100%" }}>
          <div
            style={{
              position: "relative",
              maxHeight: "210px",
              zIndex: 9999,
            }}
          >
            <List
              height={210}
              itemCount={filteredOptions.length}
              itemSize={35}
              width="100%"
              overscanCount={10}
            >
              {Row}
            </List>
          </div>
        </div>
      )}
    </div>
  );
}

export default SelectSearch;
