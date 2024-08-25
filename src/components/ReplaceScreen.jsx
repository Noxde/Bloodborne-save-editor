import { useContext, useState } from "react";
import Item from "./Item";
import { invoke } from "@tauri-apps/api";
import { drawCanvas } from "../utils/drawCanvas";
import { SaveContext } from "../context/context";
import SearchComponent from "./SearchComponent";

function ReplaceScreen({
  setSelected,
  selected,
  setReplaceScreen,
  selectedRef,
}) {
  const [replacement, setReplacement] = useState(null);
  const { setSave } = useContext(SaveContext);

  async function handleConfirm() {
    try {
      const edited = await invoke("transform_item", {
        index: selected.index,
        id: selected.id,
        newId: parseInt(replacement.id),
        articleType: selected.article_type,
      });

      setSave(edited);

      const newJson = {
        index: selected.index,
        amount: selected.amount,
        ...replacement,
      };

      const ctx = selectedRef.current.getContext("2d");
      selectedRef.current.dataset.itemId = parseInt(replacement.id);
      selectedRef.current.dataset.item = JSON.stringify(newJson);

      setSelected(newJson);
      ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
      await drawCanvas(ctx, newJson);

      setReplaceScreen(false);
    } catch (error) {
      console.log(error);
    }
  }

  return (
    <div id="replaceScreen">
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          padding: "0 3rem",
          height: "496px",
        }}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "space-between",
            alignItems: "start",
          }}
        >
          {/* To replace */}
          <div>
            <span>Replacing:</span>
            <Item item={selected} isSmall={true} />
          </div>
          <div>
            <button
              onClick={() => setReplaceScreen(false)}
              style={{ marginRight: "50px" }}
              id="cancelReplace"
            >
              Cancel
            </button>
            <button id="confirmReplace" onClick={handleConfirm}>
              Confirm
            </button>
          </div>
        </div>
        {/* List and input */}
        <SearchComponent selected={selected} setReplacement={setReplacement} />
      </div>
    </div>
  );
}

export default ReplaceScreen;
