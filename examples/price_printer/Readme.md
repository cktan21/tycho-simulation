# Price Printer

This example allows you to list all pools over a certain tvl threshold and explore
quotes from each pool.

## How to run

```shell
# Set the Infura Ethereum RPC URL
$env:RPC_URL = <your-node-rpc-url>
# Run the price_printer example with a TVL threshold of 1000
cargo run --release --example price_printer -- --tvl-threshold 1000
```
