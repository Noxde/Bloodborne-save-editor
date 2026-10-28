import { useContext, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SaveContext } from "../../context/context";
import SearchAllitems from "../../components/SearchAllitems";
import useDraw from "../../utils/useDraw";

function AddScreen({ type, setAddScreen, isStorage }) {
  const [selected, setSelected] = useState(null);
  const { setSave } = useContext(SaveContext);
  const { getType } = useDraw();

  async function handleConfirm() {
    if (!selected) return;

    try {
      console.log(selected);
      const selectedType = getType(selected.article_type);
      if (["item", "key", "chalice"].includes(selectedType)) {
        const editedSave = await invoke("add_item", {
          id: selected.id,
          quantity: 1,
          isStorage,
        });

        setSave(editedSave);
      } else {
        const editedSave = await invoke("add_armor_or_weapon", {
          id: selected.id,
          isStorage,
        });

        setSave(editedSave);
      }

      setAddScreen(false);
    } catch (error) {
      console.error(error);
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
        <SearchAllitems type="all" onChange={(x) => setSelected(x)} />
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
