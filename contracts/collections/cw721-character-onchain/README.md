# cw721 with on chain character information

This is an adaptation of the cw-nfts onchain metadata contract to
include the CoolCat character information onchain using the Metadata extension.
This collection will keep the information of all the CoolCat characters and
each character will have multiple traits and the value of each trait will be
kept in the Metadata:

```rust
pub struct Metadata {
    pub name: String,
    pub ears: Option<String>,
    pub eyes: Option<String>,
    pub mouth: Option<String>,
    pub fur_type: Option<String>,
    pub fur_color: Option<String>,
    pub tail_shape: Option<String>,
    pub frozen: bool,
}
```
