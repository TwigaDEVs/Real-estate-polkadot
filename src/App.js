import React, { useState, useEffect } from "react";
import Homepage from "./pages/Homepage";
import AllProperties from "./pages/AllProperties";
import apiPromise from "./polkadot";
import NewProperty from "./pages/NewProperty";
function App() {
  return (
    <div className="App">
      {/* <Homepage /> */}

      {/* <AllProperties /> */}
      <NewProperty />
      {/* <h1>Hello world, connected to {chain} network</h1> */}
    </div>
  );
}

export default App;
