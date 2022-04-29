use sbor::*;

use crate::misc::*;
use crate::rust::borrow::ToOwned;
use crate::rust::convert::TryFrom;
use crate::rust::fmt;
use crate::rust::str::FromStr;
use crate::rust::string::String;
use crate::rust::vec::Vec;
use crate::types::*;

/// Represents a 32-byte hash digest.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hash(pub [u8; Self::LENGTH]);

impl Hash {
    pub const LENGTH: usize = 32;

    /// Returns the lower 26 bytes.
    pub fn lower_26_bytes(&self) -> [u8; 26] {
        let mut result = [0u8; 26];
        result.copy_from_slice(&self.0[6..32]);
        result
    }

    /// Returns the lower 16 bytes.
    pub fn lower_16_bytes(&self) -> [u8; 16] {
        let mut result = [0u8; 16];
        result.copy_from_slice(&self.0[16..32]);
        result
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Computes the hash digest of a message.
pub fn hash<T: AsRef<[u8]>>(data: T) -> Hash {
    // TODO: replace with whatever hash algorithm we eventually agrees on
    // The point here is to have a single "main" hashing function in the code base

    crate::crypto::sha256(data)
}

//========
// error
//========

/// Represents an error when parsing hash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseHashError {
    InvalidHex(String),
    InvalidLength(usize),
}

#[cfg(not(feature = "alloc"))]
impl std::error::Error for ParseHashError {}

#[cfg(not(feature = "alloc"))]
impl fmt::Display for ParseHashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

//========
// binary
//========

impl TryFrom<&[u8]> for Hash {
    type Error = ParseHashError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        if slice.len() != Hash::LENGTH {
            return Err(ParseHashError::InvalidLength(slice.len()));
        }
        Ok(Self(copy_u8_array(slice)))
    }
}

impl Hash {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

scrypto_type!(Hash, ScryptoType::Hash, Vec::new());

//======
// text
//======

impl FromStr for Hash {
    type Err = ParseHashError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s).map_err(|_| ParseHashError::InvalidHex(s.to_owned()))?;
        Self::try_from(bytes.as_slice())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::string::ToString;

    #[test]
    fn test_from_to_string() {
        let s = "b177968c9c68877dc8d33e25759183c556379daa45a4d78a2b91c70133c873ca";
        let h = Hash::from_str(s).unwrap();
        assert_eq!(h.to_string(), s);
    }
}
