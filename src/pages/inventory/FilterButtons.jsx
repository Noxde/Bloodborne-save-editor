function FilterButtons({ selectedFilter }) {
  const spaceBetween = (795 - 9 * 80) / (9 + 1);
  return (
    <div className="filters">
      <div
        id="filterHover"
        style={{
          display: selectedFilter == 0 ? "none" : "block",
          left: `${
            spaceBetween + (80 + spaceBetween) * (selectedFilter - 1) - 2
          }px`,
        }}
      ></div>
      {/* Should change the naming scheme to be iterable */}
      <button
        data-index={1}
        className="filters-button"
        style={{
          background: `url(${
            process.env.PUBLIC_URL + "/assets/filters/consumables.png"
          })`,
        }}
      ></button>
      <button
        data-index={2}
        className="filters-button"
        style={{
          background: `url(${
            process.env.PUBLIC_URL + "/assets/filters/materials.png"
          })`,
        }}
      ></button>
      <button
        data-index={3}
        className="filters-button"
        style={{
          background: `url(${
            process.env.PUBLIC_URL + "/assets/filters/key.png"
          })`,
        }}
      ></button>
      <button
        data-index={4}
        className="filters-button"
        style={{
          background: `url(${
            process.env.PUBLIC_URL + "/assets/filters/right_hand.png"
          })`,
        }}
      ></button>
      <button
        data-index={5}
        className="filters-button"
        style={{
          background: `url(${
            process.env.PUBLIC_URL + "/assets/filters/left_hand.png"
          })`,
        }}
      ></button>
      <button
        data-index={6}
        className="filters-button"
        style={{
          background: `url(${
            process.env.PUBLIC_URL + "/assets/filters/armor.png"
          })`,
        }}
      ></button>
      <button
        data-index={7}
        className="filters-button"
        style={{
          background: `url(${
            process.env.PUBLIC_URL + "/assets/filters/gems.png"
          })`,
        }}
      ></button>
      <button
        data-index={8}
        className="filters-button"
        style={{
          background: `url(${
            process.env.PUBLIC_URL + "/assets/filters/runes.png"
          })`,
        }}
      ></button>
      <button
        data-index={9}
        className="filters-button"
        style={{
          background: `url(${
            process.env.PUBLIC_URL + "/assets/filters/chalices.png"
          })`,
        }}
      ></button>
    </div>
  );
}

export default FilterButtons;
