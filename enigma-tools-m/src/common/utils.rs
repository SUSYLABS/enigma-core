//! # Mutual Utils.
//! This module contain some handy utils.
//! Right now only a trait that can convert `[u8; 64]` to a 20 bytes Ethereum address
//! or a 20 bytes Ethereum address String in hex representation.

use crate::localstd::string::String;
use enigma_crypto::hash::Keccak256;
use rustc_hex::ToHex;

/// A trait to convert an object into an Ethereum Address
pub trait EthereumAddress<T, P> {
    /// This should convert the object(by hashing and slicing) into a String type 40 characters Ethereum address.
    fn address_string(&self) -> T
    where T: Sized;
    /// This should convert the object(by hashing and slicing) int a 20 byte Ethereum address.
    fn address(&self) -> P
    where P: Sized;
}

impl EthereumAddress<String, [u8; 20]> for [u8; 64] {
    // TODO: Maybe add a checksum address
    fn address_string(&self) -> String {
        let mut result: String = String::from("0x");
        let hex: String = self.keccak256()[12..32].to_hex();
        result.push_str(&hex);
        result
    }

    fn address(&self) -> [u8; 20] {
        let mut result = [0u8; 20];
        result.copy_from_slice(&self.keccak256()[12..32]);
        result
    }
}
