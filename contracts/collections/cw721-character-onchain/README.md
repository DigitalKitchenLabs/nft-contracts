# cw721 with on chain character information

This is an adaptation of the cw-nfts onchain metadata contract to
include the CoolCat character information onchain using the Metadata extension.
Each trait will have its own collection (e.g. Fur Color) and the value
of each NFT will be kept in this Metadata.

```rust
pub struct Metadata {
    pub name: String,
    pub level: u32,
    pub ear_type: Option<String>,
    pub glasses: Option<String>,
    pub fur_type: Option<String>,
    pub fur_color: Option<String>,
    pub facial_expression: Option<String>,
    pub tail_shape: Option<String>,
    pub frozen: bool,
}
```
