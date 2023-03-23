import React, { useState, useEffect } from "react";
import navButtons from "./data/NavData";
import Homepage from "./pages/Homepage";
import AllProperties from "./pages/AllProperties";
import apiPromise from "./polkadot";
function App() {
  // const [chain, setChain] = useState("");
  // useEffect(() => {
  //   apiPromise.isReady.then(() => {
  //     apiPromise.rpc.system.chain().then((chain) => {
  //       setChain(chain.toString());
  //     });
  //   });
  // }, []);
  console.log(navButtons);

  return (
    <div className="App">
      {/* <h1>Hello world, connected to {chain} network</h1> */}
      {/* <Homepage />
       */}
      <AllProperties />
    </div>
  );
}

export default App;
