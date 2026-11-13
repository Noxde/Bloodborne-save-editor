import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

const CONDITIONS = {
  charge: "Charge ATK",
  fullHp: "ATK at full HP",
  nearDeath: "ATK near death",
  openFoes: "ATK vs open foes",
};

// Side panel on the right of the Gems screen
const panelStyle = {
  flexShrink: 0,
  overflowY: "auto",
  padding: "1.5rem 1.2rem",
  borderLeft: "1px solid #6b5f49",
  background: "#0000004d",
  fontFamily: "monospace",
  fontSize: 13,
};

const signed = (n) => (n >= 0 ? `+${n}` : `${n}`);
const attack = ({ base, bonus }) => `${base ?? "-"} ${signed(bonus)}`;
const percent = (n) => (n === 0 ? "-" : `${signed(n)}%`);
const conditionName = ({ condition, all }) =>
  all ? CONDITIONS[condition] : CONDITIONS[condition].replace("ATK", "Phys. ATK");

function rows(stats) {
  const conditional = Object.fromEntries(
    stats.conditional.map((c) => [conditionName(c), percent(c.percent)]),
  );
  return {
    "Physical ATK": attack(stats.physical),
    "  Blunt": stats.blunt == null ? "-" : signed(stats.blunt),
    "  Thrust": stats.thrust == null ? "-" : signed(stats.thrust),
    "Blood ATK": attack(stats.blood),
    "Arcane ATK": attack(stats.arcane),
    "Fire ATK": attack(stats.fire),
    "Bolt ATK": attack(stats.bolt),
    "Converted to": stats.converted_to ?? "-",
    "QS Bullet Use": stats.bullets ?? "-",
    Durability: stats.durability,
    "Slow Poison": stats.slow_poison,
    "Rapid Poison": stats.rapid_poison,
    "ATK vs Beasts": `${stats.beasts}%`,
    "ATK vs Kin": `${stats.kin}%`,
    "Attr. Bonus": stats.bonus.join(" / "),
    "Attr. Requirement": stats.requirements
      .map((x) => (x === 0 ? "-" : x))
      .join(" / "),
    "Stamina Cost": percent(stats.stamina_cost),
    "Rally Potential": percent(stats.rally),
    "HP Recovery": stats.hp_regen === 0 ? "-" : signed(stats.hp_regen),
    ...conditional,
  };
}

// Stats of the weapon with its current gems, compared with the ones it had
// before the last confirmed gem change ("before > after", like the game menu)
function WeaponStatsTable({ article }) {
  const [stats, setStats] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    let cancelled = false;
    const gemEffects = article.slots.flatMap(
      (slot) => slot.gem?.effects.map(([id]) => id) ?? [],
    );

    invoke("weapon_stats", { weaponId: article.id, gemEffects })
      .then((current) => {
        if (cancelled) return;
        setError(null);
        setStats((prev) => ({ before: prev?.after ?? current, after: current }));
      })
      .catch((e) => !cancelled && setError(e));

    return () => {
      cancelled = true;
    };
  }, [article]);

  if (error) {
    return (
      <div style={panelStyle}>
        <p style={{ color: "#b84a4a" }}>{String(error)}</p>
      </div>
    );
  }
  if (!stats) return <div style={panelStyle}></div>;

  const before = rows(stats.before);
  const after = rows(stats.after);
  // Situational effects can appear or disappear between the two
  const labels = [...new Set([...Object.keys(before), ...Object.keys(after)])];
  const [str, skl, bld, arc] = stats.after.attributes;

  return (
    <div style={panelStyle}>
      <table style={{ borderCollapse: "collapse", whiteSpace: "nowrap" }}>
        <thead>
          <tr style={{ borderBottom: "1px solid #6b5f49" }}>
            <th style={{ textAlign: "left", paddingRight: "1.2rem" }}>Stat</th>
            <th style={{ textAlign: "right" }}>Before</th>
            <th style={{ padding: "0 0.6rem" }}></th>
            <th style={{ textAlign: "right" }}>After</th>
          </tr>
        </thead>
        <tbody>
          {labels.map((label) => {
            const a = before[label] ?? "-";
            const b = after[label] ?? "-";
            const changed = String(a) !== String(b);
            return (
              <tr key={label} style={{ borderBottom: "1px solid #6b5f4955" }}>
                <td style={{ whiteSpace: "pre", paddingRight: "1.2rem" }}>{label}</td>
                <td style={{ textAlign: "right" }}>{a}</td>
                <td style={{ textAlign: "center", padding: "0 0.6rem" }}>&gt;</td>
                <td
                  style={{
                    textAlign: "right",
                    color: changed ? "#e8c170" : undefined,
                  }}
                >
                  {b}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
      <p style={{ fontSize: 12, opacity: 0.7, maxWidth: "36ch" }}>
        Calculated for STR {str} / SKL {skl} / BLT {bld} / ARC {arc}.
        Situational effects are listed but not added to the attack.
      </p>
    </div>
  );
}

export default WeaponStatsTable;
