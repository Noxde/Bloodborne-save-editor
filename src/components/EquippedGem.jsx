import { useEffect, useRef } from "react";
import useDraw from "../utils/useDraw";
import ShapeSelector from "./ShapeSelector";

function EquippedGem({
  gem,
  shape,
  setSelected,
  setRef,
  isStorage,
  articleType,
  articleIndex,
  index,
}) {
  const canvasRef = useRef();
  const { getGemPath, getUnique, loadImage } = useDraw();

  useEffect(() => {
    if (gem != null) {
      const {
        effects,
        info: { level },
        shape,
      } = gem;

      const canvas = canvasRef?.current;
      if (canvas) {
        const ctx = canvas.getContext("2d");
        ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
        const unique = getUnique(effects[0][0], shape, shape);
        const path = getGemPath(effects, shape, level, unique);
        loadImage(path).then((img) => {
          ctx.drawImage(img, 0, 0, 175, 175);
        });
      }
    }
  }, []);

  return (
    <>
      {/* TODO: Hover */}
      <div
        style={{
          position: "relative",
          padding: "1rem",
          backgroundImage: "url(/assets/gems/gem_bg.png)",
          backgroundSize: "contain",
        }}
        onClick={() => {
          setSelected({
            gem,
            index,
          });
          setRef(canvasRef);
        }}
      >
        <ShapeSelector
          isStorage={isStorage}
          articleType={articleType}
          articleIndex={articleIndex}
          shape={shape}
          slotIndex={index}
        />
        <canvas
          ref={canvasRef}
          width="175px"
          height="175px"
          style={{
            display: "block",
          }}
        ></canvas>
      </div>
    </>
  );
}

export default EquippedGem;
