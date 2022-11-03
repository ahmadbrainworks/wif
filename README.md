[![version-badge][]][version] [![license-badge][]][license] [![rust-version-badge][]][rust-version]

Wallet import format decoder and encoder for bitcoin written in RUST.

# Rust Version Policy

This crate only supports the current stable version of Rust, patch releases may
use new features at any time.

# License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


## Usage

```rust
use wif::{encode, decode};

println!("{:?}",decode("93SewNjbsxJLEgRmF6QYvhtMbztGeUjmr3ng6yVrD47r6E8bUvi","testnet")); // DecodedWif { network: "testnet", private_key: "F4A194DF2E1442B8F82B8D762BF8EC1C7E7E054798A5162F99CEB7DF4683B1FB", compressed: false }

 println!("{:?}",encode("a70253dbfc1540647bd27626706c60c9745a0f1948ff07463a1505d9f0ba35ec","testnet",true,)); //  EncodedWif { network: "testnet", wif: "cTBLzVDsLsNRxhyngnYMyaFVw9CiipCnnJSQ6cgW9NYr6Hkt1Gjm", compressed: true }
```



[version-badge]: https://img.shields.io/badge/crates.io-v0.1.0-orange
[version]: https://crates.io/crates/wif
[license-badge]: https://img.shields.io/badge/license-MIT%2FApache%202.0-blue
[license]: #license
[rust-version-badge]: https://img.shields.io/badge/rust-latest%20stable-blueviolet.svg?style=flat-square
[rust-version]: #rust-version-policy

[WIF]: https://en.bitcoin.it/wiki/Wallet_import_format
