import { invoke } from "@tauri-apps/api/core";
import { basename } from "@tauri-apps/api/path";
import { useState } from "react";
import * as dialog from "@tauri-apps/plugin-dialog";
import {
  AndroidFs,
  AndroidPublicGeneralPurposeDir,
  AndroidProgressNotificationIconType,
} from "tauri-plugin-android-fs-api";

function Nav({ setLoading, setSave, save }) {
  const [name, setName] = useState("");

  async function readFile() {
    try {
      const [selected] = await AndroidFs.showOpenFilePicker({
        localOnly: true,
      });
      const bytes = await AndroidFs.readFile(selected);

      if (save) setSave(null);
      setLoading(true);

      const parsedSave = await invoke("make_save", {
        bytes,
      });
      setLoading(false);
      setSave(parsedSave);
      setName(await AndroidFs.getName(selected));
    } catch (error) {
      console.error(error);
      await dialog.message(
        "Could not parse file, make sure it is a decrypted save file",
        {
          title: "Failed to parse save",
          kind: "error",
        },
      );
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
