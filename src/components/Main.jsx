import React from "react";
import { useLocation, Routes, Route } from "react-router-dom";
import SideBar from "./SideBar";
import Inventory from "./Inventory";
import Stats from "./Stats";
import Character from "./Character";
import { SaveContext } from "../context/context";
import { ItemsProvider } from "../context/itemsContext";

const Main = ({ save, setSave, loading }) => {
  const location = useLocation();

  return (
    <main
      style={{
        gridTemplateColumns: `minmax(160px, 200px) 805px ${
          location.pathname.match(/storage|\/$/) != null ? "1fr" : ""
        }`,
      }}
    >
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
            This save editor works with decrypted save files, meaning that you
            can't use the savefile you get from exporting it directly from your
            playstation. If you don't know how to decrypt a save click{" "}
            <a
              style={{ textDecoration: "underline" }}
              href="https://github.com/Noxde/Bloodborne-save-editor/wiki/"
              target="_blank"
              rel="noreferrer"
            >
              here
            </a>{" "}
            to learn more.
          </div>
        ) : null}
      </SaveContext.Provider>
    </main>
  );
};

export default Main;
