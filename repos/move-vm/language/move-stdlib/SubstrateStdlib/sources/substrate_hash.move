/// Cryptographic hashes:
/// - Keccak-256: see https://keccak.team/keccak.html
///
/// In addition, SHA2-256 and SHA3-256 are available in `std::hash`. Note that SHA3-256 is a variant of Keccak: it is
/// NOT the same as Keccak-256.
///
/// Non-cryptograhic hashes:
/// - SipHash: an add-rotate-xor (ARX) based family of pseudorandom functions created by Jean-Philippe Aumasson and Daniel J. Bernstein in 2012
module substrate::substrate_hash {
    /// Returns the (non-cryptographic) SipHash of `bytes`. See https://en.wikipedia.org/wiki/SipHash
    native public fun sip_hash(bytes: vector<u8>): u64;

    /// Returns the Keccak-256 hash of `bytes`.
    native public fun keccak256(bytes: vector<u8>): vector<u8>;

    /// Returns the SHA2-512 hash of `bytes`.
    native public fun sha2_512(bytes: vector<u8>): vector<u8>;

    /// Returns the SHA3-512 hash of `bytes`.
    native public fun sha3_512(bytes: vector<u8>): vector<u8>;

    /// Returns the RIPEMD-160 hash of `bytes`.
    ///
    /// WARNING: Only 80-bit security is provided by this function. This means an adversary who can compute roughly 2^80
    /// hashes will, with high probability, find a collision x_1 != x_2 such that RIPEMD-160(x_1) = RIPEMD-160(x_2).
    native public fun ripemd160(bytes: vector<u8>): vector<u8>;

    /// Returns the BLAKE2B-256 hash of `bytes`.
    native public fun blake2b_256(bytes: vector<u8>): vector<u8>;
}
