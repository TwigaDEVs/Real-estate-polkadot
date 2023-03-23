import React from "react";
import Navbar from "../components/Navbar";
import PropertyCard from "../components/PropertyCard";
import properties from "../data/PropertyData";

const AllProperties = () => {
  console.log(properties);
  return (
    <div>
      <Navbar />
      <br />
      <div className="w3-row-padding w3-auto w3-stretch">
        {properties.map((property) => (
          <div className="w3-col m4" key={property.property_name}>
            <PropertyCard
              name={property.property_name}
              image={property.property_image}
              price={property.property_price}
              type={property.property_type}
            />
          </div>
        ))}
        {properties.forEach((property) => console.log(property))}
      </div>
    </div>
  );
};

export default AllProperties;
