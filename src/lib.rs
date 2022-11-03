//! WIF encoder and Decoder written in rust lang, guided by https://en.bitcoin.it/wiki/Wallet_import_format
use bs58;
use hex::ToHex;

#[derive(Debug)]
pub struct EncodedWif {
    pub network: String,
    pub wif: String,
    pub compressed: bool,
}

#[derive(Debug)]
pub struct DecodedWif {
    pub network: String,
    pub private_key: String,
    pub compressed: bool,
}
/// Takes wif and network type ("mainnet" or "testnet") and convert to to private_key if plaintext("hex format")
pub fn decode(wif: &str, network: &str) -> DecodedWif {
    let compressed = &bs58::decode(wif)
        .with_alphabet(bs58::Alphabet::BITCOIN)
        .into_vec()
        .unwrap()[1..33];
    let uncompressed = &bs58::decode(wif)
        .with_alphabet(bs58::Alphabet::BITCOIN)
        .into_vec()
        .unwrap()[1..33];

    let key = match wif.len() {
        52 => compressed,
        51 => uncompressed,
        _ => panic!("Invalid wif: {}", wif.len()),
    };
    let comp = match wif.len() {
        52 => true,
        51 => false,
        _ => panic!(),
    };
    if network == "mainnet" {
        return DecodedWif {
            network: "mainnet".to_string(),
            private_key: key.encode_hex_upper::<String>(),
            compressed: comp,
        };
    } else {
        return DecodedWif {
            network: "testnet".to_string(),
            private_key: key.encode_hex_upper::<String>(),
            compressed: comp,
        };
    }
}

/// Encode private_key(in hex format) to a WIF (wallet import format)
pub fn encode(private_key: &str, network: &str, compressed: bool) -> EncodedWif {
    if compressed == true {
        let extended_key_ = match network {
            "mainnet" => format!("{}{}{}", "80", private_key, "01"),
            "testnet" => format!("{}{}{}", "ef", private_key, "01"),
            _ => panic!("ERROR:: invalid network selected"),
        };
        let extended_key_bytes: Vec<u8> = hex::decode(&extended_key_).unwrap();
        let extended_key: Vec<u8> =
            hex::decode(openssl::sha::sha256(&extended_key_bytes).encode_hex_upper::<String>())
                .unwrap();
        let epkey = hex::decode(format!(
            "{}{}",
            extended_key_,
            &openssl::sha::sha256(&extended_key).encode_hex_upper::<String>()[0..8]
        ))
        .unwrap();
        return EncodedWif {
            network: network.to_string(),
            wif: bs58::encode(epkey).into_string(),
            compressed,
        };
    } else {
        let extended_key_ = match network {
            "mainnet" => format!("{}{}", "80", private_key),
            "testnet" => format!("{}{}", "ef", private_key),
            _ => panic!("ERROR:: invalid network selected"),
        };

        let extended_key_bytes: Vec<u8> = hex::decode(&extended_key_).unwrap();
        let extended_key: Vec<u8> =
            hex::decode(openssl::sha::sha256(&extended_key_bytes).encode_hex_upper::<String>())
                .unwrap();
        let epkey = hex::decode(format!(
            "{}{}",
            extended_key_,
            &openssl::sha::sha256(&extended_key).encode_hex_upper::<String>()[0..8]
        ))
        .unwrap();
        return EncodedWif {
            network: network.to_string(),
            wif: bs58::encode(epkey).into_string(),
            compressed,
        };
    }
}

/// # Examples
/// ```rust
/// println!(
///     "{:?}",
///     decode(
///         "93SewNjbsxJLEgRmF6QYvhtMbztGeUjmr3ng6yVrD47r6E8bUvi",
///         "testnet"
///     )
/// );
/// encode(
///     "04b29da18785419336ddda569b714500d996d9c5d3221fa148a4126d571b163a",
///     "mainnet",
///     true,
/// );
/// ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{:?}",
            decode(
                "93SewNjbsxJLEgRmF6QYvhtMbztGeUjmr3ng6yVrD47r6E8bUvi",
                "testnet"
            )
        );
        println!(
            "{:?}",
            encode(
                "a70253dbfc1540647bd27626706c60c9745a0f1948ff07463a1505d9f0ba35ec",
                "testnet",
                true,
            )
        );
    }
}
