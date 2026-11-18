import { useContext, useEffect, useRef, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import Item from "../../components/Item";
import EditUpgrade from "../../components/EditUpgrade";
import { ImagesContext } from "../../context/imagesContext";
import EquippedGem from "./EquippedGem";
import ChangeGemScreen from "./ChangeGemScreen";
import WeaponStatsTable from "./WeaponStatsTable";

function EquippedGems() {
  const { images } = useContext(ImagesContext);

  const {
    state: { selected, isStorage },
  } = useLocation();
  const [article, setArticle] = useState(selected);
  const [selectedRef, setSelectedRef] = useState(null);
  const [editScreen, setEditScreen] = useState(false);
  const [changeScreen, setChangeScreen] = useState(false);
  const [selectedGem, setSelectedGem] = useState(null);
  const nav = useNavigate();

  // Stats panel visibility, remembered between visits
  const [showStats, setShowStats] = useState(() => {
    try {
      return localStorage.getItem("showWeaponStats") !== "false";
    } catch {
      return true;
    }
  });
  function toggleStats() {
    setShowStats((prev) => {
      try {
        localStorage.setItem("showWeaponStats", String(!prev));
      } catch {}
      return !prev;
    });
  }

  useEffect(() => {
    if (!selected) {
      nav("/");
    }
    console.log(selected);
  }, [selected]);

  return (
    <>
      {changeScreen ? (
        <ChangeGemScreen
          slotIndex={selectedGem.index}
          article={article}
          setArticle={setArticle}
          setSelected={setSelectedGem}
          setScreen={setChangeScreen}
          isStorage={isStorage}
        />
      ) : null}
      {editScreen ? (
        <EditUpgrade
          setSelected={setSelectedGem}
          selected={selectedGem.gem}
          selectedRef={selectedRef}
          setEditScreen={setEditScreen}
          isStorage={isStorage}
          equipped={article}
          slot={selectedGem.index}
          confirmCb={(newGem) => {
            setArticle((prev) => {
              const index = prev.slots.findIndex(
                (x) => x.gem?.id === selectedGem.gem.id,
              );
              prev.slots[index].gem = newGem;
              return JSON.parse(JSON.stringify(prev));
            });

            setSelectedGem(null);
          }}
        />
      ) : null}
      <div
        style={{
          gridColumn: "2/4",
          display: "flex",
          background: `url(${images.backgrounds["statsBg.png"].src})`,
          backgroundSize: "cover",
          overflow: "hidden",
        }}
      >
        {/* Weapon, gems and buttons */}
        <div
          style={{
            flex: 1,
            minWidth: 0,
            display: "flex",
            flexDirection: "column",
            padding: "2rem",
            overflowY: "auto",
          }}
        >
          {/* margin auto centers the content but lets it scroll when it doesn't fit */}
          <div
            style={{
              margin: "auto 0",
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
            }}
          >
            {/* Selected Weapon */}
            <div
              style={{
                marginBottom: "5rem",
              }}
            >
              <Item item={article} index={0} />
            </div>
            {/* Gems */}
            <div
              style={{
                display: "flex",
                position: "relative",
              }}
            >
              <div
                className="selected-slot"
                style={{
                  display: selectedGem !== null ? "block" : "none",
                  left: `${selectedGem?.index * 207}px`,
                }}
              ></div>
              {article.slots.map((slot, i) => (
                <EquippedGem
                  gem={slot?.gem}
                  shape={slot.shape}
                  setRef={setSelectedRef}
                  setSelected={setSelectedGem}
                  isStorage={isStorage}
                  article={article}
                  setArticle={setArticle}
                  index={i}
                  key={i}
                />
              ))}
            </div>
            {/* Buttons */}
            <div
              style={{
                marginTop: "5rem",
              }}
            >
              <button onClick={() => nav("/")}>Back</button>
              <button
                style={{ margin: "0 2rem" }}
                onClick={() => {
                  setChangeScreen(true);
                  setShowStats(false);
                }}
                disabled={!selectedGem}
              >
                Change
              </button>
              <button
                onClick={() => {
                  setEditScreen(true);
                  setShowStats(false);
                }}
                disabled={!selectedGem?.gem}
              >
                Edit
              </button>
              {article.type_family === "Weapon" ? (
                <button
                  style={{
                    marginLeft: "2rem",
                    zIndex: 991,
                    position: "relative",
                  }}
                  onClick={toggleStats}
                >
                  {showStats ? "Hide Stats" : "Show Stats"}
                </button>
              ) : null}
            </div>
          </div>
        </div>
        {article.type_family === "Weapon" ? (
          // Hidden instead of unmounted so the before/after comparison is kept
          <WeaponStatsTable article={article} hidden={!showStats} />
        ) : null}
      </div>
    </>
  );
}

export default EquippedGems;
