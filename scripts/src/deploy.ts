import * as path from "path";
import yargs from "yargs/yargs";
import { createLCDClient, createWallet } from "./cli";
import { storeCodeWithConfirm, instantiateWithConfirm } from "./tx";

const argv = yargs(process.argv)
  .options({
    network: {
      type: "string",
      demandOption: true,
    },
    "code-id": {
      type: "number",
      demandOption: false,
    },
  })
  .parseSync();

(async function () {
  const terra = createLCDClient(argv["network"]);
  const deployer = createWallet(terra);

  let codeId = argv["code-id"];
  if (!codeId) {
    codeId = await storeCodeWithConfirm(
      deployer,
      path.join(__dirname, "../../artifacts/cw_stargate_query.wasm")
    );
    console.log(`Code uploaded! codeId: ${codeId}`);
  }

  const result = await instantiateWithConfirm(deployer, codeId, {});
  const address = result.logs[0].eventsByType.instantiate_contract.contract_address[0];
  console.log(`Contract instantiated! address: ${address}`);
})();
