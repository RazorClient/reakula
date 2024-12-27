
/// These are the helper functions for the reakula
pub(crate) mod macros;

pub mod common;

pub mod presets;

/// Starting of with the phase0 specs
/// This module will contain all the specs for the phase0 of the eth2.0
/// This is the very starting of our journey to the ehtereum
pub mod phase0 {

    /// The beacon state of ethereum
    pub mod beacon_state;

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

    pub mod containers {
        pub mod fork;
        pub use fork::*;

        pub mod fork_data;
        pub use fork_data::*;

        pub mod checkpoint;
        pub use checkpoint::*;

        pub mod validator;
        pub use validator::*;

        pub mod attestation_data;
        pub use attestation_data::*;

        pub mod indexed_attestation;
        pub use indexed_attestation::*;

        pub mod pending_attestation;
        pub use pending_attestation::*;

        pub mod eth1_data;
        pub use eth1_data::*;

        pub mod historical_batch;
        pub use historical_batch::*;

        pub mod deposit_message;
        pub use deposit_message::*;

        pub mod deposit_data;
        pub use deposit_data::*;

        pub mod beacon_block_header;
        pub use beacon_block_header::*;

        pub mod signing_data;
        pub use signing_data::*;

        pub mod proposer_slashing;
        pub use proposer_slashing::*;

        pub mod attester_slashing;
        pub use attester_slashing::*;

        pub mod attestation;
        pub use attestation::*;

        pub mod deposit;
        pub use deposit::*;

        pub mod voluntary_exit;
        pub use voluntary_exit::*;

        pub mod beacon_block_body;
        pub use beacon_block_body::*;

        pub mod beacon_block;
        pub use beacon_block::*;

        pub mod signed_voluntary_exit;
        pub use signed_voluntary_exit::*;

        pub mod signed_beacon_block;
        pub use signed_beacon_block::*;

        pub mod signed_beacon_block_header;
        pub use signed_beacon_block_header::*;
    }
}