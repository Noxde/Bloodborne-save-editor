import { Link } from "react-router-dom";

function SideBar() {
  return (
    <div id="sideBar">
      <ul>
        <li>
          <Link to={"/"}>Inventory</Link>
        </li>
        <li>
          <Link to={"/storage"}>Storage</Link>
        </li>
        <li>
          <Link to={"/stats"}>Stats</Link>
        </li>
        <li>
          <Link to={"/character"}>Character</Link>
        </li>
      </ul>
    </div>
  );
}

export default SideBar;
