import { useContext, useEffect, useRef, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import Item from "../../components/Item";
import EditUpgrade from "../../components/EditUpgrade";
import { ImagesContext } from "../../context/imagesContext";
import EquippedGem from "./EquippedGem";
import useDraw from "../../utils/useDraw";
import { SaveContext } from "../../context/context";
import ChangeGemScreen from "./ChangeGemScreen";
import WeaponStatsTable from "./WeaponStatsTable";

function EquippedGems() {
  const { images } = useContext(ImagesContext);
  const { save } = useContext(SaveContext);
  const { getGemPath, loadImage } = useDraw();

  const {
    state: { selected, isStorage },
  } = useLocation();
  const [article, setArticle] = useState(selected);
  const [selectedRef, setSelectedRef] = useState(null);
  const [editScreen, setEditScreen] = useState(false);
  const [changeScreen, setChangeScreen] = useState(false);
  const [selectedGem, setSelectedGem] = useState(null);
  const nav = useNavigate();

  // Shrink the gem slots (207px each) when the column is narrower than them
  const columnRef = useRef(null);
  const [gemsZoom, setGemsZoom] = useState(1);
  const slotCount = article?.slots?.length ?? 0;
  useEffect(() => {
    const column = columnRef.current;
    if (!column || slotCount === 0) return;
    const observer = new ResizeObserver(([entry]) => {
      setGemsZoom(Math.min(1, entry.contentRect.width / (slotCount * 207)));
    });
    observer.observe(column);
    return () => observer.disconnect();
  }, [slotCount]);

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
                (x) => x.gem?.id === selectedGem.gem.id
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
          ref={columnRef}
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
                zoom: gemsZoom,
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
                }}
                disabled={!selectedGem}
              >
                Change
              </button>
              <button
                onClick={() => {
                  setEditScreen(true);
                }}
                disabled={!selectedGem?.gem}
              >
                Edit
              </button>
            </div>
          </div>
        </div>
        {article.type_family === "Weapon" ? (
          <WeaponStatsTable article={article} />
        ) : null}
      </div>
    </>
  );
}

export default EquippedGems;
