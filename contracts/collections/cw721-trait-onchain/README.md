# cw721 with on chain trait information

This is an adaptation of the cw-nfts onchain metadata contract to
include the CoolCat trait information onchain using the Metadata extension.
All traits will be kept in one collection and will have the trait type (e.g. Fur Color),
the trait value (e.g. red) and the trait rarity (e.g. common).

```rust
pub struct Metadata {
    pub trait_type: String,
    pub trait_value: String,
    pub trait_rarity: String,
}
```
