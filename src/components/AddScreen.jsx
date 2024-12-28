import { useContext, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { SaveContext } from "../context/context";
import SearchAllitems from "./SearchAllitems";

function AddScreen({ type, setAddScreen, isStorage }) {
  const [selected, setSelected] = useState(null);
  const { setSave } = useContext(SaveContext);

  async function handleConfirm() {
    try {
      if (selected) {
        const editedSave = await invoke("add_item", {
          id: selected.id,
          quantity: 1,
          isStorage,
        });

        setSave(editedSave);
      }

      setAddScreen(false);
    } catch (error) {
      console.log(error);
    }
  }

  return (
    <div id="replaceScreen">
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-around",
          alignItems: "center",
          padding: "0 3rem",
          height: "100%",
        }}
      >
        {/* List and input */}
        <SearchAllitems type={type} onChange={(x) => setSelected(x)} />
        <div>
          <button
            onClick={() => setAddScreen(false)}
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
    </div>
  );
}

export default AddScreen;
