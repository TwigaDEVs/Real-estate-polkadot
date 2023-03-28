import flipper from "./flipper.json";

import { ContractPromise } from "@polkadot/api-contract";
const { ApiPromise, WsProvider } = require("@polkadot/api");

async function main() {
  const provider = new WsProvider("wss://rococo-contracts-rpc.polkadot.io");
  const api = await ApiPromise.create({ provider });
  const address = "5CN6NtuRRYsteHFvTgxn6cUffWbcD1qJ6WRPXFW9JYH6Cx8i";
  const contract = new ContractPromise(api, flipper, address);

  // maximum gas to be consumed for the call. if limit is too small the call will fail.
  const gasLimit = 5000n * 1000000n;
  // a limit to how much Balance to be used to pay for the storage created by the contract call
  // if null is passed, unlimited balance can be used
  const storageDepositLimit = null;
  // balance to transfer to the contract account. use only with payable messages, will fail otherwise.
  // formerly know as "endowment"
  // (We perform the send from an account, here using Alice's address)
  const { gasRequired, storageDeposit, result, output } =
    await contract.query.get(
      "5CN6NtuRRYsteHFvTgxn6cUffWbcD1qJ6WRPXFW9JYH6Cx8i",
      {
        gasLimit,
        storageDepositLimit,
      }
    );

  // The actual result from RPC as `ContractExecResult`
  console.log(result.toHuman());

  // the gas consumed for contract execution
  console.log(gasRequired.toHuman());

  // check if the call was successful
  if (result.isOk) {
    // output the return value
    console.log("Success", output.toHuman());
  } else {
    console.error("Error", result.asErr);
  }
}
main().catch(console.error);
console.log(flipper);
