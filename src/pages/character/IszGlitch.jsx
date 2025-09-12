import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import * as dialog from "@tauri-apps/plugin-dialog";

function IszGlitch() {
  const [isz, setIsz] = useState([]);

  useEffect(() => {
    invoke("get_isz").then((d) => setIsz(d));
  }, []);

  return (
    <div
      style={{
        fontSize: "25px",
        marginTop: "5px",
        display: "flex",
        justifyContent: "space-between",
      }}
    >
      <span>
        Isz status: {isz.map((x) => x.toString(16).toUpperCase()).join(" ")}
      </span>
      <button
        style={{
          width: "174px",
          fontSize: "25px",
          padding: "0 15px",
          backgroundSize: "100% 100%",
        }}
        className="buttonBg"
        onClick={async () => {
          const message = await invoke("fix_isz");
          setIsz(await invoke("get_isz"));
          await dialog.message(message);
        }}
      >
        Fix isz
      </button>
    </div>
  );
}

export default IszGlitch;
