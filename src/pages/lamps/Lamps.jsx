import { useContext, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SaveContext } from "../../context/context";
import lamps from "../../lamps.json";
import "./lamps.css";

const areaOrder = [
  "Central Yharnam", "Cathedral Ward", "Upper Cathedral Ward", "Old Yharnam",
  "Hemwick Charnel Lane", "Forbidden Woods", "Byrgenwerth", "Yahar’gul",
  "Cainhurst", "Abandoned Old Workshop", "Lecture Building", "Nightmare Frontier",
  "Nightmare of Mensis", "Hunter’s Nightmare", "Research Hall", "Fishing Hamlet",
];

const entries = lamps.map((lamp, index) => ({ ...lamp, index }));

function LampToggle({ label, lamps, values, busy, onChange }) {
  const checkbox = useRef(null);
  const count = lamps.filter((lamp) => values[lamp.index]).length;
  const all = count === lamps.length;

  useEffect(() => {
    if (checkbox.current) checkbox.current.indeterminate = count > 0 && !all;
  }, [count, all]);

  return (
    <label className="lamp-toggle">
      <input
        ref={checkbox}
        type="checkbox"
        disabled={busy}
        checked={all}
        onChange={(event) => onChange(lamps, event.target.checked)}
      />
      <span>{label}</span>
      {lamps.length > 1 && <small>{count}/{lamps.length}</small>}
    </label>
  );
}

function Lamps() {
  const { save } = useContext(SaveContext);
  const [values, setValues] = useState([]);
  const [error, setError] = useState("");
  const [busy, setBusy] = useState(true);

  useEffect(() => {
    let active = true;
    setBusy(true);
    invoke("get_lamp_flags")
      .then((flags) => {
        if (active) {
          setValues(flags);
          setError("");
        }
      })
      .catch((error) => { if (active) setError(String(error)); })
      .finally(() => { if (active) setBusy(false); });
    return () => { active = false; };
  }, [save]);

  async function changeLamps(group, enabled) {
    setBusy(true);
    try {
      const flags = await invoke("set_lamp_flags", {
        ids: group.map((lamp) => lamp.eventFlag),
        enabled,
      });
      setValues(flags);
      setError("");
    } catch (error) {
      setError(String(error));
    } finally {
      setBusy(false);
    }
  }

  return (
    <section className="lamps-page">
      <h1>Lamps</h1>
      <p>
        Toggle all lamps, an area, or individual lamps. Early travel to boss
        lamps can cause problems; the DLC headstone may require separate activation.
      </p>
      {error && <p role="alert">{error}</p>}
      <LampToggle label="All lamps" lamps={entries} values={values} busy={busy} onChange={changeLamps} />
      <div className="lamp-areas">
        {areaOrder.map((area) => {
          const group = entries.filter((lamp) => lamp.area === area);
          if (!group.length) return null;
          return (
            <section className="lamp-area" key={area}>
              <LampToggle label={area} lamps={group} values={values} busy={busy} onChange={changeLamps} />
              <div className="lamp-items">
                {group.map((lamp) => (
                  <LampToggle key={lamp.eventFlag} label={lamp.name} lamps={[lamp]} values={values} busy={busy} onChange={changeLamps} />
                ))}
              </div>
            </section>
          );
        })}
      </div>
    </section>
  );
}

export default Lamps;
