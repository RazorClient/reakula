
/// These are the helper functions for the reakula
pub(crate) mod macros;

/// Starting of with the phase0 specs
/// This module will contain all the specs for the phase0 of the eth2.0
/// This is the very starting of our journey to the ehtereum
pub mod phase0 {

    /// These are the custom types of the ethereum
    pub mod primitives {
        pub mod slot; // A slot number
        pub mod epoch; // An epoch number
        pub mod committee_index; // A committee index at a slot
        pub mod validator_index; // A validator registry index
        pub mod gwei; // A amount in Gwei
        pub mod root; // A Merkle root
        pub mod hash32; // A 256-bit hash
        pub mod version; // A fork version number
        pub mod domain_type; // A digest of the current fork data
        pub mod fork_digest; // A signature domain
        pub mod domain; // A signature domain
        pub mod bls_pubkey; // A BLS12-381 public key
        pub mod bls_signature; // A BLS12-381 signature
    }
}