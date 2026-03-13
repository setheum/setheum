//! Base58 address format converter for Substrate and Move accounts.

use anyhow::{bail, Result};
use move_core_types::account_address::AccountAddress;

use super::PUB_KEY_LEN;

// Maximum supported Base58 address length in bytes
const BASE58_LEN: usize = PUB_KEY_LEN;

/// Convert Base58 address string to Move address structure.
/// In case if such conversion is not possible, return error.
/// ```
/// use move_vm_support::base58_address::base58_to_move_address;
/// let base58_address = "BH9wjrYwKi76DbU6LC1X4VEeevUTkaKgxGnaFNFaqZA";
/// let move_address = base58_to_move_address(base58_address).unwrap();
/// assert_eq!(
///    "0x02A212DE6A9DFA3A69E22387ACFBAFBB1A9E591BD9D636E7895DCFC8DE05F331",
///   format!("{:#X}", move_address)
/// );
/// ```
pub fn base58_to_move_address(base58: &str) -> Result<AccountAddress> {
    // Decoded format: <address>
    //  Size in bytes:    32
    let decoded_base58 = bs58::decode(base58).into_vec()?;

    // Check if the length is valid and figure out the address type length.
    if decoded_base58.len() != BASE58_LEN {
        bail!(
            "unsupported base58 address length (base58): {}",
            decoded_base58.len()
        );
    }

    AccountAddress::from_bytes(decoded_base58).map_err(anyhow::Error::msg)
}

/// Convert Base58 address to Move address string.
pub fn base58_to_move_address_string(base58: &str) -> Result<String> {
    Ok(format!("{:#X}", base58_to_move_address(base58)?))
}

/// Convert Move address to base58 address string
/// Read more about the address structure: https://docs.substrate.io/reference/address-formats/
///
/// For now this function is supporting only one-byte address types, with one universal address type
/// for all Move accounts and it's 42. We can't do otherwise as we've no information about the
/// network we're currently compiling for.
///
/// Move developers should be aware, that information about any fails during the compilation of
/// Move code will point to the addresses converted to the 42 address type.
/// ```
/// use move_core_types::account_address::AccountAddress;
/// use move_vm_support::base58_address::move_address_to_base58_string;
/// let move_address = "0x02A212DE6A9DFA3A69E22387ACFBAFBB1A9E591BD9D636E7895DCFC8DE05F331";
/// let base58_address = move_address_to_base58_string(&AccountAddress::from_hex_literal(move_address).unwrap());
/// assert_eq!(
///   "BH9wjrYwKi76DbU6LC1X4VEeevUTkaKgxGnaFNFaqZA",
///  base58_address
/// );
/// ```
pub fn move_address_to_base58_string(addr: &AccountAddress) -> String {
    let mut base58_address = [0; BASE58_LEN];
    base58_address[0..PUB_KEY_LEN].copy_from_slice(&addr.to_vec());
    bs58::encode(base58_address).into_string()
}

/// Converts a base58 address string to a Substrate SS58 address string.
pub fn base58_string_to_ss588_string(addr: &str) -> Result<String> {
    let move_address = base58_to_move_address(addr)?;
    Ok(crate::ss58_address::move_address_to_ss58_string(
        &move_address,
    ))
}

/// Converts a Substrate SS58 address string into a base58 string.
pub fn ss58_string_to_base58_string(addr: &str) -> Result<String> {
    let move_address = crate::ss58_address::ss58_to_move_address(addr)?;
    Ok(move_address_to_base58_string(&move_address))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base58_to_move_correct() {
        let substrate_address_base58 = "BH9wjrYwKi76DbU6LC1X4VEeevUTkaKgxGnaFNFaqZA";
        let move_address = base58_to_move_address_string(substrate_address_base58).unwrap();

        assert_eq!(
            (move_address.len() - 2) / 2, // 2 hex chars per byte
            PUB_KEY_LEN
        );

        assert_eq!(
            "0x02A212DE6A9DFA3A69E22387ACFBAFBB1A9E591BD9D636E7895DCFC8DE05F331",
            move_address
        );

        let substrate_address_base58 = "7s3xJXvnK3hZCE1ZvuHyhUxifWbQdiLa3QffBccBaQy";
        let move_address = base58_to_move_address_string(substrate_address_base58).unwrap();

        assert_eq!(
            (move_address.len() - 2) / 2, // 2 hex chars per byte
            PUB_KEY_LEN
        );

        assert_eq!(
            "0x01C213DE6B7CBD4B58D33165BEBACDCA2B8D672C68E545F87A6EBFB7ED13E432",
            move_address
        );
    }

    #[test]
    fn test_base58_to_move_fail() {
        let substrate_address = "7s3xJXvnK3hZCE1ZvuHyhUxifWbQdiLa3QffBccBa"; // too short
        assert!(base58_to_move_address_string(substrate_address).is_err());
    }

    #[test]
    fn move_address_to_base58_string_correct() {
        let move_address = "0x02A212DE6A9DFA3A69E22387ACFBAFBB1A9E591BD9D636E7895DCFC8DE05F331";
        let substrate_address =
            move_address_to_base58_string(&AccountAddress::from_hex_literal(move_address).unwrap());

        assert_eq!(
            "BH9wjrYwKi76DbU6LC1X4VEeevUTkaKgxGnaFNFaqZA",
            substrate_address
        );

        let move_address = "0x01C213DE6B7CBD4B58D33165BEBACDCA2B8D672C68E545F87A6EBFB7ED13E432";
        let substrate_address =
            move_address_to_base58_string(&AccountAddress::from_hex_literal(move_address).unwrap());

        assert_eq!(
            "7s3xJXvnK3hZCE1ZvuHyhUxifWbQdiLa3QffBccBaQy",
            substrate_address
        );
    }

    #[test]
    #[should_panic]
    fn move_address_to_base58_string_fail() {
        let move_address = "0x02A212DE6A9DFA3A69E22387ACFBAFBB1A9E591BD9D636E7895DCFC8DE05F3310A"; // too long
        let _substrate_addr =
            move_address_to_base58_string(&AccountAddress::from_hex_literal(move_address).unwrap());
    }

    #[test]
    fn base58_to_ss58_correct() {
        let base58_address = "AbygL37RheNZv327cMvZPqKYLLkZ6wqWYexRxgNiZyeP";
        let ss58_address = base58_string_to_ss588_string(base58_address).unwrap();
        assert_eq!(
            "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
            ss58_address
        );

        let base58_address = "AjtNsBf2JLsVANbtsqLTLrGz5JxDRosGT8XXqn1cPvSd";
        let ss58_address = base58_string_to_ss588_string(base58_address).unwrap();
        assert_eq!(
            "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
            ss58_address
        );

        // This one is Bob's address for further move-project based testings.
        let base58_address = "AbygL37RheNZv327cMvZPqKYLLkZ6wqWYexRxgNiZyeP";
        let ss58_address = base58_string_to_ss588_string(base58_address).unwrap();
        assert_eq!(
            "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
            ss58_address
        );
    }

    #[test]
    fn ss58_to_base58_correct() {
        let ss58_address = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
        let base58_address = ss58_string_to_base58_string(ss58_address).unwrap();
        assert_eq!(
            "AbygL37RheNZv327cMvZPqKYLLkZ6wqWYexRxgNiZyeP",
            base58_address
        );

        let ss58_address = "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y";
        let base58_address = ss58_string_to_base58_string(ss58_address).unwrap();
        assert_eq!(
            "AjtNsBf2JLsVANbtsqLTLrGz5JxDRosGT8XXqn1cPvSd",
            base58_address
        );

        // This one is Bob's address for further move-project based testings.
        let ss58_address = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
        let base58_address = ss58_string_to_base58_string(ss58_address).unwrap();
        assert_eq!(
            "AbygL37RheNZv327cMvZPqKYLLkZ6wqWYexRxgNiZyeP",
            base58_address
        );
    }
}
