import "./App.css";

import { useEffect, useState } from "react";
import Nav from "./components/Nav";
import { BrowserRouter as Router } from "react-router-dom";
import { dialog, invoke, shell } from "@tauri-apps/api";
import Main from "./components/Main";
import { ImagesProvider } from "./context/imagesContext";

function App() {
  const [save, setSave] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    async function checkUpdate() {
      try {
        const req = await fetch(
          "https://api.github.com/repos/Noxde/Bloodborne-save-editor/releases/latest"
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
      } catch (err) {
        console.error(err);
      }
    }

    checkUpdate();
  }, []);

  useEffect(() => {
    document.addEventListener("contextmenu", (e) => e.preventDefault());
  }, []);

  useEffect(() => {
    document.addEventListener("keydown", (e) => {
      console.log(e);
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
