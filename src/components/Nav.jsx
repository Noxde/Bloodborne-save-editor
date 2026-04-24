import { invoke } from "@tauri-apps/api/core";
import { basename } from "@tauri-apps/api/path";
import { useState } from "react";
import * as dialog from "@tauri-apps/plugin-dialog";

function Nav({ setLoading, setSave, save, setShowMenu }) {
  const [name, setName] = useState("");

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
      <button
        id="openSave"
        onClick={() => {
          setShowMenu((prev) => !prev);
        }}
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-menu-icon lucide-menu"
        >
          <path d="M4 5h16" />
          <path d="M4 12h16" />
          <path d="M4 19h16" />
        </svg>
      </button>
      <span>{name}</span>
      <button disabled={save == null ? true : false} onClick={saveChanges}>
        Save
      </button>
    </nav>
  );
}

export default Nav;
