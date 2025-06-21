import { useContext } from "react";
import { Link } from "react-router-dom";
import { SaveContext } from "../../context/context";

function SideBar() {
  const { save } = useContext(SaveContext);

  return (
    <div id="sideBar">
      <ul>
        <li
          className={
            save && document.location.pathname.match(/^\/$/) ? "selected" : ""
          }
        >
          <Link to={"/"}>Inventory</Link>
        </li>
        <li
          className={
            save && document.location.pathname.match(/storage/)
              ? "selected"
              : ""
          }
        >
          <Link to={"/storage"}>Storage</Link>
        </li>
        <li
          className={
            save && document.location.pathname.match(/stats/) ? "selected" : ""
          }
        >
          <Link to={"/stats"}>Stats</Link>
        </li>
        <li
          className={
            save && document.location.pathname.match(/character/)
              ? "selected"
              : ""
          }
        >
          <Link to={"/character"}>Character</Link>
        </li>
        <li
          className={
            save && document.location.pathname.match(/bosses/) ? "selected" : ""
          }
        >
          <Link to={"/bosses"}>Bosses</Link>
        </li>
      </ul>
    </div>
  );
}

export default SideBar;
