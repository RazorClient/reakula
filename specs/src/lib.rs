
/// These are the helper functions for the reakula
pub(crate) mod macros;

/// Starting of with the phase0 specs
/// This module will contain all the specs for the phase0 of the eth2.0
/// This is the very starting of our journey to the ehtereum
pub mod phase0 {

    /// These are the custom types of the ethereum
    pub mod primitives {
        /// A slot number
        pub mod slot; 
        pub use slot::*;

        /// An epoch number
        pub mod epoch; 
        pub use epoch::*;

        /// A committee index at a slot
        pub mod committee_index; 
        pub use committee_index::*;

        /// A validator registry index
        pub mod validator_index; 
        pub use validator_index::*;

        /// A amount in Gwei
        pub mod gwei; 
        pub use gwei::*;

        /// A Merkle root
        pub mod root; 
        pub use root::*;

        /// A 256-bit hash
        pub mod hash32; 
        pub use hash32::*;

        /// A fork version number
        pub mod version; 
        pub use version::*;

        /// A digest of the current fork data
        pub mod domain_type; 
        pub use domain_type::*;

        /// A signature domain
        pub mod fork_digest; 
        pub use fork_digest::*;

        /// A signature domain
        pub mod domain; 
        pub use domain::*;

        /// A BLS12-381 public key
        pub mod bls_pubkey; 
        pub use bls_pubkey::*;

        /// A BLS12-381 signature
        pub mod bls_signature; 
        pub use bls_signature::*;
    }

    pub mod constants;
}