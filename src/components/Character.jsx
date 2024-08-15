import { useContext } from "react";
import { SaveContext } from "../context/context";

function Character() {
  const { save } = useContext(SaveContext);

  return (
    <div>
      Echoes: {save.stats.find((x) => x.name === "Echoes").value} Insight:{" "}
      {save.stats.find((x) => x.name === "Insight").value}
    </div>
  );
}

export default Character;
