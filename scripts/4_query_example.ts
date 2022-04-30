import yargs from "yargs/yargs";
import { createLCDClient } from "./helpers";

const argv = yargs(process.argv)
  .options({
    network: {
      type: "string",
      demandOption: true,
    },
    "contract-addr": {
      type: "string",
      demandOption: true,
    },
    "validator-addr": {
      type: "string",
      demandOption: false,
    },
  })
  .parseSync();

(async function () {
  const terra = createLCDClient(argv["network"]);

  const response = await terra.wasm.contractQuery(argv["contract-addr"], {
    validator: argv["validator-addr"],
  });
  console.log(response);
})();
