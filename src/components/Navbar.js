import React from "react";
import navButtons from "../data/NavData";

const Navbar = () => {
  return (
    <div>
      <div className="w3-bar w3-teal">
        <span className="w3-bar-item w3-large">REMS</span>
        <div className="w3-right">
          {navButtons.map((navButton) => {
            return (
              <div key={navButton} className="w3-text-white w3-bar-item">
                {navButton}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};

export default Navbar;
