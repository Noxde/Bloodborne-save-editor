import { useContext, useEffect } from "react";
import { useLocation, useNavigate, Routes, Route } from "react-router-dom";
import SideBar from "./SideBar";
import Inventory from "../inventory/Inventory";
import Stats from "../stats/Stats";
import Character from "../character/Character";
import { SaveContext } from "../../context/context";
import { ItemsProvider } from "../../context/itemsContext";
import { ImagesContext } from "../../context/imagesContext";
import EquippedGems from "./EquippedGems";
import Bosses from "../bosses/Bosses";
import Flags from "../flags/Flags";
import Lamps from "../lamps/Lamps";
import About from "../about/About";
import "../lamps/lamps.css";

const Main = ({ save, setSave, loading }) => {
  const location = useLocation();
  const navigate = useNavigate();
  useEffect(() => {
    try {
      if (localStorage.getItem("bb-desktop-about-version") !== __APP_VERSION__) {
        navigate("/about", { replace: true });
        localStorage.setItem("bb-desktop-about-version", __APP_VERSION__);
      }
    } catch { navigate("/about", { replace: true }); }
  }, [navigate]);
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

          <Routes>
              <Route path="/about" element={<About />} />
              {save != null && <>
              <Route
                path="/"
                element={
                  <ItemsProvider>
                    <Inventory
                      key={"inventory"}
                      inv={save.inventory}
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
                      inv={save.storage}
                      isStorage={true}
                    />
                  </ItemsProvider>
                }
              />
              <Route path="/stats" element={<Stats />} />
              <Route path="/character" element={<Character />} />
              <Route
                path="/equippedGems"
                element={
                  <ItemsProvider>
                    <EquippedGems />
                  </ItemsProvider>
                }
              ></Route>
              <Route path="/bosses" element={<Bosses />}></Route>
              <Route path="/flags" element={<Flags />}></Route>
              <Route path="/lamps" element={<Lamps />}></Route>
              </>}
            </Routes>

          {save == null && !loading && location.pathname !== "/about" ? (
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
