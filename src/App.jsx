import "./App.css";

import { invoke, dialog, fs } from "@tauri-apps/api";
import { useEffect, useRef, useState, useContext } from "react";
import Item from "./components/Item";
import Nav from "./components/Nav";
import SideBar from "./components/SideBar";
import Inventory from "./components/Inventory";
import { SaveContext } from "./context/context";

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
      <main>
        <SideBar />
        <SaveContext.Provider value={{ save, setSave }}>
          {loading ? <div>Loading</div> : null}

          {save != null ? <Inventory /> : null}
        </SaveContext.Provider>
      </main>
    </div>
  );
}

export default App;
