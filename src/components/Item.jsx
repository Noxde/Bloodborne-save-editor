import { useEffect, useRef } from "react";
import useDraw from "../utils/useDraw";

function Item({ index, item, isSmall, className, ...props }) {
  const canvasRef = useRef(null);
  const { drawCanvas } = useDraw();

  useEffect(() => {
    const canvas = canvasRef?.current;
    if (!canvas) return;
    const ctx = canvasRef.current.getContext("2d");
    drawCanvas(ctx, item, isSmall);
  }, [item, isSmall]);

  return isSmall ? (
    <canvas
      data-index={index}
      data-item-id={item.id}
      data-item={JSON.stringify(item)}
      width={526}
      height={90}
      ref={canvasRef}
      style={{ display: "block", marginBottom: "1px" }}
      {...props}
    ></canvas>
  ) : (
    <div className={className}>
      <canvas
        data-index={index}
        data-item-id={item.id}
        data-item-type={
          item?.article_type?.toLowerCase() || item.upgrade_type.toLowerCase()
        }
        data-item={JSON.stringify(item)}
        width={795}
        height={90}
        ref={canvasRef}
        style={{ display: "block", marginBottom: "1px" }}
        {...props}
      ></canvas>
    </div>
  );
}

export default Item;
