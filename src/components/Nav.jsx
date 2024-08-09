import { invoke, dialog, fs } from "@tauri-apps/api";
import { basename } from "@tauri-apps/api/path";
import { useState } from "react";

function Nav({ setLoading, setSave, save }) {
  const [name, setName] = useState("");

  async function readFile() {
    try {
      const selectedPath = await dialog.open({
        multiple: false,
        title: "Character data",
      });
      if (!selectedPath) return;
      setLoading(true);

      const fileContents = await fs.readBinaryFile(selectedPath);
      const parsedSave = await invoke("make_save", {
        path: selectedPath,
        name: "Control",
      });
      setLoading(false);
      setSave(parsedSave);
      setName(await basename(selectedPath));

      console.log(selectedPath);
      console.log(fileContents);
      console.log(parsedSave);
    } catch (error) {
      console.error(error);
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

      console.log(saved);
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
