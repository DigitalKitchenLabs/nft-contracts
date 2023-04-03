# cw721 with on chain trait information

This is an adaptation of the cw-nfts onchain metadata contract to
include the trait information onchain using the Metadata extension.
Each trait will have its own collection (e.g. Fur Color) and the value
of each NFT will be kept in this Metadata.

```rust
pub struct Metadata {
    pub value: Option<String>
}
```
