import { ApiPromise, WsProvider } from "@polkadot/api";
const wsProvider = new WsProvider("wss://rpc.polkadot.io");
const apiPromise = ApiPromise.create({ provider: wsProvider });
export default apiPromise;
