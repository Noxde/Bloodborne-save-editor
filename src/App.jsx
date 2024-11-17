import "./App.css";

import { useEffect, useState } from "react";
import Nav from "./components/Nav";
import SideBar from "./components/SideBar";
import Inventory from "./components/Inventory";
import { SaveContext } from "./context/context";
import { Routes, BrowserRouter as Router, Route } from "react-router-dom";
import Stats from "./components/Stats";
import Character from "./components/Character";
import { ItemsProvider } from "./context/itemsContext";
import { dialog, invoke, shell } from "@tauri-apps/api";

function App() {
  const [save, setSave] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    async function checkUpdate() {
      const req = await fetch(
        "  https://api.github.com/repos/Noxde/Bloodborne-save-editor/releases/latest"
      );
      const { tag_name, html_url } = await req.json();
      const currentVersion = await invoke("get_version");

      if (tag_name > currentVersion) {
        const ok = await dialog.confirm("New update available.", {
          title: "Update available",
          type: "info",
          okLabel: "Go to github",
        });
        if (ok) {
          shell.open(html_url);
        }
      }
    }

    checkUpdate();
  }, []);

  useEffect(() => {
    document.addEventListener("contextmenu", (e) => e.preventDefault());
  }, []);

  return (
    <div className="App">
      <Router>
        <Nav setLoading={setLoading} save={save} setSave={setSave} />
        <main>
          <SideBar />
          <SaveContext.Provider value={{ save, setSave }}>
            {loading ? <div>Loading</div> : null}

            {save != null ? (
              <Routes>
                <Route
                  path="/"
                  element={
                    <ItemsProvider>
                      <Inventory
                        key={"inventory"}
                        articles={save.inventory.articles}
                        isStorage={false}
                      />
                    </ItemsProvider>
                  }
                />
                <Route
                  path="/storage"
                  element={
                    <ItemsProvider>
                      <Inventory
                        key={"storage"}
                        articles={save.storage.articles}
                        isStorage={true}
                      />
                    </ItemsProvider>
                  }
                />
                <Route path="/stats" element={<Stats />} />
                <Route path="/character" element={<Character />} />
              </Routes>
            ) : null}

            {save == null && !loading ? (
              <div
                style={{
                  gridColumn: "2/4",
                  justifySelf: "center",
                  padding: "2.5rem",
                }}
              >
                This save editor works with decrypted save files, meaning that
                you can't use the savefile you get from exporting it directly
                from your playstation. If you don't know how to decrypt a save
                click{" "}
                <a
                  style={{ textDecoration: "underline" }}
                  href="https://github.com/Noxde/Bloodborne-save-editor/wiki/"
                  target="_blank"
                  rel="noreferrer"
                >
                  here
                </a>{" "}
                {/* TODO: Make a wiki page on decrypting saves */}
                to learn more.
              </div>
            ) : null}
          </SaveContext.Provider>
        </main>
      </Router>
    </div>
  );
}

export default App;
