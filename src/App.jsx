import logo from "./logo.svg";
import "./App.css";

import { invoke, dialog, fs } from "@tauri-apps/api";
import { useEffect, useRef, useState } from "react";
import Item from "./components/Item";
import Nav from "./components/Nav";
import SideBar from "./components/SideBar";

function App() {
  const [save, setSave] = useState(null);
  const [loading, setLoading] = useState(false);

  /**
   * <nav/>
   * <main/> routes; sidebar
   */

  return (
    <div className="App">
      <Nav setLoading={setLoading} save={save} setSave={setSave} />
      <main id="content">
        <SideBar />
        {/* <p style={{ cursor: "pointer" }} onClick={readFile}>
        Open
        </p> */}

        {loading ? <div>Loading</div> : null}

        {save != null ? (
          <div style={{ overflowY: "scroll" }}>
            <div
              style={{ width: "795px", height: "90px", background: "grey" }}
            ></div>
            {save.inventory.articles.map((x) => (
              <Item key={x.index} item={x} />
            ))}
          </div>
        ) : null}
      </main>
    </div>
  );
}

export default App;
