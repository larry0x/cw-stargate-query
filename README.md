# cw-stargate-query

Am example on how to use Stargate raw queries in CosmWasm contracts. This is useful when the query is not supported by the `cosmwasm_std::QueryRequest` API.

## Usage

The dockerized optimizer provided by CosmWasm does not have dependencies such as `protoc` set up, so we need to manually compile and optimize the contract.

Compile the optimizer:

```bash
git clone https://github.com/WebAssembly/binaryen.git
cd binaryan
cmake . && make
```

Compile the contract:

```bash
cd /path/to/cw-stargate-query
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked
```

Optimize:

```bash
/path/to/binaryen/bin/wasm-opt -Os ./target/wasm32-unknown-unknown/release/cw_stargate_query.wasm -o ./artifacts/cw_stargate_query.wasm
```

Deploy:

```bash
cd script
npm install
ts-node 1_deploy.ts --network mainnet|testnet|localterra --key keyname
```

## License

Contents of this repository are open source under the [MIT License](./License)
