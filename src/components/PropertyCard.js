import React from "react";

const PropertyCard = ({ name, image, price, type }) => {
  return (
    <div className="w3-card">
      <div
        className="w3-container"
        style={{
          backgroundImage: `url(${image})`,
          backgroundSize: "cover",
          backgroundPosition: "center",
          height: "100px",
        }}
      ></div>
      <div>
        {name} | {price} | {type}
        <br />
        <button className="w3-button w3-teal">View</button>
      </div>
    </div>
  );
};

export default PropertyCard;
