//! Opaque payload for the `zcash-sign-batch` Registry Type.
//!
//! The PCZT crate owns the versioned Postcard request schema. This registry
//! wrapper only associates those bytes with their UR type, keeping domain and
//! policy validation out of the generic UR SDK.

use alloc::vec::Vec;

use crate::{
    error::{URError, URResult},
    registry_types::{RegistryType, ZCASH_SIGN_BATCH},
    traits::RegistryItem,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ZcashSignBatch {
    data: Vec<u8>,
}

impl ZcashSignBatch {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}

impl RegistryItem for ZcashSignBatch {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_SIGN_BATCH
    }
}

impl TryFrom<Vec<u8>> for ZcashSignBatch {
    type Error = URError;

    fn try_from(data: Vec<u8>) -> URResult<Self> {
        Ok(Self::new(data))
    }
}

impl TryInto<Vec<u8>> for ZcashSignBatch {
    type Error = URError;

    fn try_into(self) -> URResult<Vec<u8>> {
        Ok(self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn preserves_postcard_payload() {
        let payload = vec![1, 2, 3, 4];
        let registry = ZcashSignBatch::try_from(payload.clone()).unwrap();
        assert_eq!(registry.get_data(), payload);
        assert_eq!(TryInto::<Vec<u8>>::try_into(registry).unwrap(), payload);
    }
}
