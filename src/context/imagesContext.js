import React, { createContext, useState, useEffect } from "react";
import allImages from "./allImages.json";

export const ImagesContext = createContext();

export const ImagesProvider = ({ children }) => {
  const [loading, setLoading] = useState(true);
  const [images, setImages] = useState({});
  const { itemImages, backgroundImages } = allImages;

  useEffect(() => {
    const fetchData = async () => {
      const itemsLoaded = await Promise.all(
        itemImages.map((x) =>
          loadImage(`${process.env.PUBLIC_URL}/assets/itemImages/${x}`)
        )
      );
      const backgroundsLoaded = await Promise.all(
        backgroundImages.map((x) =>
          loadImage(`${process.env.PUBLIC_URL}/assets/itemsBg/${x}`)
        )
      );

      setImages({
        items: itemImages.reduce((acc, name, i) => {
          acc[name] = itemsLoaded[i];

          return acc;
        }, {}),
        backgrounds: backgroundImages.reduce((acc, name, i) => {
          acc[name] = backgroundsLoaded[i];

          return acc;
        }, {}),
      });
      setLoading(false);
    };

    fetchData();
  }, []);

  function loadImage(url) {
    return new Promise((resolve, reject) => {
      let imageObj = new Image();
      imageObj.onload = () => resolve(imageObj);
      imageObj.onerror = (e) => {
        // If the image fails to load, use the default image
        reject("Failed to load image");
      };
      imageObj.src = url;
    });
  }

  return (
    <ImagesContext.Provider
      value={{
        images,
        loading,
      }}
    >
      {children}
    </ImagesContext.Provider>
  );
};
