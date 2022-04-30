# NFTizer | NFT Minter on NEAR Protocol

This is a ready-to-use smart contract for minting NFTs on NEAR blockchain. Below is hwo you may use it. I'm assuming you already familiar with NEAR and have installed Rust blockchain and near-cli.

## Smart Contract Sample Usage

### Build

```
env 'RUSTFLAGS=-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
```

### Deploy

```
$ near deploy --wasmFile target/wasm32-unknown-unknown/release/near_spring_nft.wasm --accountId nftizer.mhassanist.testnet
```

### Intialize the contract (call one time only)

```
$ near call nftizer.mhassanist.testnet  new_default_meta '{"owner_id": "'mhassanist.testnet'"}' --accountId mhassanist.testnet
```

### Minting

The `receiver_id` param is the account that this minted NFT goes to.

```
near call nftizer.mhassanist.testnet nft_mint '{"token_id": "Yom3", "receiver_id": "'msaudi.testnet'", "token_metadata": { "title": "KidPhotos3", "description": "Beautiful Kids NFTs", "media": "https://bafybeiab33alpecxhqr4bciie7jfgwcgbc7yj27utdrcfa7uzx46mjff5q.ipfs.nftstorage.link/", "copies": 1}}' --accountId mhassanist.testnet --deposit 0.1
```
