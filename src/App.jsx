import "./App.css";

import { useState } from "react";
import Nav from "./components/Nav";
import SideBar from "./components/SideBar";
import Inventory from "./components/Inventory";
import { SaveContext } from "./context/context";
import { Routes, BrowserRouter as Router, Route } from "react-router-dom";
import Stats from "./components/Stats";
import Character from "./components/Character";
import { ItemsProvider } from "./context/itemsContext";

function App() {
  const [save, setSave] = useState(null);
  const [loading, setLoading] = useState(false);

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
                        <Inventory />
                      </ItemsProvider>
                    }
                  />
                  <Route path="/stats" element={<Stats />} />
                  <Route path="/character" element={<Character />} />
                </Routes>
              ) : null}
            </SaveContext.Provider>
        </main>
      </Router>
    </div>
  );
}

export default App;
