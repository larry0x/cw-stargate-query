import yargs from "yargs/yargs";
import { createLCDClient } from "./cli";

const argv = yargs(process.argv)
  .options({
    network: {
      type: "string",
      demandOption: true,
    },
    "contract-address": {
      type: "string",
      demandOption: true,
    },
    "query-msg": {
      type: "string",
      demandOption: true,
    },
  })
  .parseSync();

(async function () {
  const terra = createLCDClient(argv["network"]);
  const response = await terra.wasm.contractQuery(
    argv["contract-address"],
    JSON.parse(argv["query-msg"])
  );
  console.log("Query response:", response);
})();
