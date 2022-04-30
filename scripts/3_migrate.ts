import * as fs from "fs";
import * as path from "path";
import yargs from "yargs/yargs";
import { MsgMigrateContract } from "@terra-money/terra.js";
import * as keystore from "./keystore";
import {
  createLCDClient,
  createWallet,
  waitForConfirm,
  sendTxWithConfirm,
  storeCodeWithConfirm,
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
    "contract-address": {
      type: "string",
      demandOption: true,
    },
    msg: {
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
      default: "../artifacts/multiquery.wasm",
    },
  })
  .parseSync();

(async function () {
  const terra = createLCDClient(argv["network"]);
  const admin = await createWallet(terra, argv["key"], argv["key-dir"]);
  const msg = argv["msg"]
    ? JSON.parse(fs.readFileSync(path.resolve(argv["msg"]), "utf8"))
    : {};

  let codeId = argv["code-id"];
  if (!codeId) {
    codeId = await storeCodeWithConfirm(admin, path.resolve(argv["binary"]));
    console.log(`Code uploaded! codeId: ${codeId}`);
    await waitForConfirm("Proceed to migrate contract?");
  }

  const { txhash } = await sendTxWithConfirm(admin, [
    new MsgMigrateContract(
      admin.key.accAddress,
      argv["contract-address"],
      codeId,
      msg
    ),
  ]);
  console.log(`Contract migrated! Txhash: ${txhash}`);
})();
