# Internft is a cross-chain NFT network of the Web3 world in the future

[![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](LICENSE) [![GitLab Status](https://gitlab.parity.io/parity/substrate/badges/master/pipeline.svg)](https://gitlab.parity.io/parity/substrate/pipelines) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](docs/CONTRIBUTING.adoc)

Internft is built on Substrate and constructed as a parachain to link to Polkadot's ecology and share its security consensus. Internft is a cross-chain NFT network, which provides fast and efficient NFT infrastructure services for Polkadot's other parachains and Ethereum ecology, aims to drive the digital asset ecosystem.

## Note

Now we are mainly testing the functions of block generation, transfer, staking, etc. We will open more codes later when we are ready.

## Running from Source

### Building
```bash
git clone https://github.com/internfts/nft-node.git
cd nft-node
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and subcommands:

```sh
./target/release/nft-node -h
```

