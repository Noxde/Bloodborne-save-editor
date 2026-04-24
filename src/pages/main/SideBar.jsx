import { useContext } from "react";
import { Link } from "react-router-dom";

import { invoke } from "@tauri-apps/api/core";
import { basename } from "@tauri-apps/api/path";
import { useState } from "react";
import * as dialog from "@tauri-apps/plugin-dialog";
import {
  AndroidFs,
  AndroidPublicGeneralPurposeDir,
  AndroidProgressNotificationIconType,
} from "tauri-plugin-android-fs-api";

function SideBar({ save, setSave, show }) {
  async function readFile() {
    try {
      const [selected] = await AndroidFs.showOpenFilePicker({
        localOnly: true,
      });
      const bytes = await AndroidFs.readFile(selected);

      if (save) setSave(null);
      // setLoading(true);

      const parsedSave = await invoke("make_save", {
        bytes,
      });
      // setLoading(false);
      setSave(parsedSave);
      // setName(await AndroidFs.getName(selected));
    } catch (error) {
      console.error(error);
      await dialog.message(
        "Could not parse file, make sure it is a decrypted save file",
        {
          title: "Failed to parse save",
          kind: "error",
        },
      );
      // setLoading(false);
    }
  }

  return (
    <div id="sideBar" className={show ? "show" : ""}>
      <ul>
        <li onClick={readFile}>Open</li>
        <li
          className={
            save && document.location.pathname.match(/^\/$/) ? "selected" : ""
          }
        >
          <Link to={"/"}>Inventory</Link>
        </li>
        <li
          className={
            save && document.location.pathname.match(/storage/)
              ? "selected"
              : ""
          }
        >
          <Link to={"/storage"}>Storage</Link>
        </li>
        <li
          className={
            save && document.location.pathname.match(/stats/) ? "selected" : ""
          }
        >
          <Link to={"/stats"}>Stats</Link>
        </li>
        <li
          className={
            save && document.location.pathname.match(/character/)
              ? "selected"
              : ""
          }
        >
          <Link to={"/character"}>Character</Link>
        </li>
        <li
          className={
            save && document.location.pathname.match(/bosses/) ? "selected" : ""
          }
        >
          <Link to={"/bosses"}>Bosses</Link>
        </li>
        <li
          className={
            save && document.location.pathname.match(/flags/) ? "selected" : ""
          }
        >
          <Link to={"/flags"}>Flags</Link>
        </li>
      </ul>
    </div>
  );
}

export default SideBar;
