import React from "react";
import Navbar from "../components/Navbar";

const NewProperty = () => {
  return (
    <div>
      <Navbar />
      <br />
      <form className="w3-auto w3-container w3-card" style={{ width: "40rem" }}>
        <div className="w3-panel w3-padding w3-text-large">Add Asset</div>
        <label>Property Id</label>
        <input className="w3-input w3-border w3-round" />
        <label>Property Id</label>
        <input className="w3-input w3-border w3-round" />
        <label>property name</label>
        <input className="w3-input w3-border w3-round" />
        <label>property Location</label>
        <input className="w3-input w3-border w3-round" />
        <label>Property Value</label>
        <input className="w3-input w3-border w3-round" />
        <label>Token Price</label>
        <input className="w3-input w3-border w3-round" />
        <label>Token Symbol</label>
        <input className="w3-input w3-border w3-round" />
        <label>Token Name</label>
        <input className="w3-input w3-border w3-round" />
      </form>
    </div>
  );
};

export default NewProperty;
