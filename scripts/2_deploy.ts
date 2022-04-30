import * as path from "path";
import yargs from "yargs/yargs";
import * as keystore from "./keystore";
import {
  createLCDClient,
  createWallet,
  waitForConfirm,
  storeCodeWithConfirm,
  instantiateWithConfirm,
} from "./helpers";

const argv = yargs(process.argv)
  .options({
    network: {
      type: "string",
      demandOption: true,
    },
    key: {
      type: "string",
      demandOption: true,
    },
    "key-dir": {
      type: "string",
      demandOption: false,
      default: keystore.DEFAULT_KEY_DIR,
    },
    admin: {
      type: "string",
      demandOption: false,
    },
    "code-id": {
      type: "number",
      demandOption: false,
    },
    binary: {
      type: "string",
      demandOption: false,
      default: "../artifacts/cw_stargate_query.wasm",
    },
  })
  .parseSync();

(async function () {
  const terra = createLCDClient(argv["network"]);
  const deployer = await createWallet(terra, argv["key"], argv["key-dir"]);

  let codeId = argv["code-id"];
  if (!codeId) {
    codeId = await storeCodeWithConfirm(deployer, path.resolve(argv["binary"]));
    console.log(`Code uploaded! Code ID: ${codeId}`);
    await waitForConfirm("Proceed to deploy contract?");
  }

  const result = await instantiateWithConfirm(
    deployer,
    argv["admin"] ? argv["admin"] : deployer.key.accAddress,
    codeId,
    {}
  );
  const address =
    result.logs[0].eventsByType["instantiate_contract"]["contract_address"][0];
  console.log(`Contract instantiated! Address: ${address}`);
})();
