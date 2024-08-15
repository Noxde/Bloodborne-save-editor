import { useContext } from "react";
import { SaveContext } from "../context/context";

function Stats() {
  const { save } = useContext(SaveContext);

  return (
    <div style={{ display: "flex", flexWrap: "wrap" }}>
      {save.stats
        .filter((x) => x.name !== "Echoes" && x.name !== "Insight")
        .map((x) => (
          <div style={{ marginRight: "5px" }}>
            {x.name}: {x.value}
          </div>
        ))}
    </div>
  );
}

export default Stats;
