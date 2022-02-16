# cw-stargate-query

Am example on how to use Stargate raw queries in CosmWasm contracts. This is useful when the query is not supported by the `cosmwasm_std::QueryRequest` API.

## Usage

Install Protobuf compiler:

```bash
brew install protobuf         // macOS
apt install protobuf-compiler // Ubuntu
```

Install the [Rust plugin](https://github.com/stepancheg/rust-protobuf/tree/v2.27.1/protobuf-codegen):

```bash
cargo install protobuf-codegen
```

Generate Rust code from Protobuf:

```bash
make gen-rust
```

Compile the contract:

```bash
make build
```

Deploy the contract:

```bash
cd script
npm install
ts-node src/deploy --network mainnet|testnet|localterra --code-id <code-id>
```

Query the contract:

```bash
ts-node src/query \
  --network mainnet|testnet|localterra \
  --contract-address terra1234...abcd \
  --query-msg '{address:"terra1234....abcd"}'
```

## License

Contents of this repository are open source under the [MIT License](./License)