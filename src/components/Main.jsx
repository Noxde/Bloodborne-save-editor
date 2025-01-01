import { useContext } from "react";
import { useLocation, Routes, Route } from "react-router-dom";
import SideBar from "./SideBar";
import Inventory from "./Inventory";
import Stats from "./Stats";
import Character from "./Character";
import { SaveContext } from "../context/context";
import { ItemsProvider } from "../context/itemsContext";
import { ImagesContext } from "../context/imagesContext";

const Main = ({ save, setSave, loading }) => {
  const location = useLocation();
  const { loading: loadingImages } = useContext(ImagesContext);

  return (
    <>
      <div
        style={{
          transition: "opacity",
          background: "#1b1b26",
          position: "absolute",
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "center",
          height: "100%",
          width: "100%",
          top: 0,
          zIndex: 99999,
        }}
        className={loadingImages ? "" : "fade-out"}
      >
        <img src="./assets/icon.png" width="200px" alt="" />
        <div className="spinner"></div>
      </div>
      <main
        style={{
          gridTemplateColumns: `200px 805px ${
            location.pathname.match(/storage|\/$/) != null ? "1fr" : ""
          }`,
        }}
      >
        <SaveContext.Provider value={{ save, setSave }}>
          <SideBar />
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
              can't use the savefile you get from exporting it directly from
              your playstation. If you don't know how to decrypt a save click{" "}
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
    </>
  );
};

export default Main;
