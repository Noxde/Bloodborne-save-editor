import "./App.css";

import { useEffect, useState } from "react";
import Nav from "./components/Nav";
import { BrowserRouter as Router } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import Main from "./pages/main/Main";
import { ImagesProvider } from "./context/imagesContext";
import * as dialog from "@tauri-apps/plugin-dialog";
import * as shell from "@tauri-apps/plugin-shell";
import { check } from "@tauri-apps/plugin-updater";
import { UpdateModal } from "./Update";

function App() {
  const [save, setSave] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    document.addEventListener("contextmenu", (e) => e.preventDefault());
  }, []);

  useEffect(() => {
    document.addEventListener("keydown", (e) => {
      const { code, ctrlKey } = e;
      if (code === "Equal" && ctrlKey) {
        document.body.style.zoom = Number(document.body.style.zoom) + 0.1;
      } else if (code === "Minus" && ctrlKey) {
        document.body.style.zoom = Number(document.body.style.zoom) - 0.1;
      }
    });
  }, []);

  useEffect(() => {
    updateZoom();

    const scalingQuery = window.matchMedia(`(min-width: 2000px)`);

    function updateZoom() {
      if (window.innerWidth > 2000) {
        document.body.style.zoom = 2;
      } else {
        document.body.style.zoom = 1;
      }
    }

    scalingQuery.addEventListener("change", updateZoom);

    return () => {
      scalingQuery.removeEventListener("change", updateZoom);
    };
  }, []);

  return (
    <div className="App">
      <UpdateModal />
      <Router>
        <Nav setLoading={setLoading} save={save} setSave={setSave} />
        <ImagesProvider>
          <Main save={save} setSave={setSave} loading={loading} />
        </ImagesProvider>
      </Router>
    </div>
  );
}

export default App;
