import { invoke } from "@tauri-apps/api/core";
import * as dialog from "@tauri-apps/plugin-dialog";

function Appearance() {
  return (
    <div
      style={{
        fontSize: "25px",
        marginTop: "5px",
        display: "flex",
        justifyContent: "space-between",
      }}
    >
      <button
        className="buttonBg"
        style={{
          padding: "0 15px",
          fontSize: "inherit",
          backgroundSize: "100% 100%",
        }}
        onClick={async () => {
          try {
            const path = await dialog.save({
              title: "Save face file",
            });

            if (path) {
              const success = await invoke("export_appearance", {
                path,
              });

              await dialog.message(success);
            }
          } catch (error) {
            console.error(error);
          }
        }}
      >
        Export face
      </button>
      <button
        className="buttonBg"
        style={{
          padding: "0 15px",
          fontSize: "inherit",
          backgroundSize: "100% 100%",
        }}
        onClick={async () => {
          try {
            const path = await dialog.open({
              title: "Select a face file",
            });

            if (path) {
              const success = await invoke("import_appearance", {
                path,
              });

              await dialog.message(success);
            }
          } catch (error) {
            console.error(error);
            await dialog.message(error, {
              type: "error",
            });
          }
        }}
      >
        Import face
      </button>
    </div>
  );
}

export default Appearance;
