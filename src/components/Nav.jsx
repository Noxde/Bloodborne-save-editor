import { invoke } from "@tauri-apps/api/core";
import { basename } from "@tauri-apps/api/path";
import { useState } from "react";
import * as dialog from "@tauri-apps/plugin-dialog";

function Nav({ setLoading, setSave, save }) {
  const [name, setName] = useState("");

  async function readFile() {
    try {
      const selectedPath = await dialog.open({
        multiple: false,
        title: "Character data",
      });
      if (!selectedPath) return;
      if (save) setSave(null);
      setLoading(true);

      const parsedSave = await invoke("make_save", {
        path: selectedPath,
      });
      setLoading(false);
      setSave(parsedSave);
      setName(await basename(selectedPath));
    } catch (error) {
      console.error(error);
      setLoading(false);
    }
  }

  async function saveChanges() {
    try {
      const path = await dialog.save({
        title: "Save changes",
        defaultPath: name,
      });

      const saved = await invoke("save", {
        save: JSON.stringify(save),
        path: path,
      });
      await dialog.message(saved);
    } catch (error) {
      console.log(error);
    }
  }

  return (
    <nav className="nav">
      <button id="openSave" onClick={readFile}>
        Open
      </button>
      <span>{name}</span>
      <button disabled={save == null ? true : false} onClick={saveChanges}>
        Save
      </button>
    </nav>
  );
}

export default Nav;
