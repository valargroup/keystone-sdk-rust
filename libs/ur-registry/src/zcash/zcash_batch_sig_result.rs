//! Opaque payload for the `zcash-batch-sig-result` Registry Type.
//!
//! The PCZT crate owns the versioned Postcard response schema. This registry
//! wrapper only associates those bytes with their UR type.

use alloc::vec::Vec;

use crate::{
    error::{URError, URResult},
    registry_types::{RegistryType, ZCASH_BATCH_SIG_RESULT},
    traits::RegistryItem,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ZcashBatchSigResult {
    data: Vec<u8>,
}

impl ZcashBatchSigResult {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}

impl RegistryItem for ZcashBatchSigResult {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_BATCH_SIG_RESULT
    }
}

impl TryFrom<Vec<u8>> for ZcashBatchSigResult {
    type Error = URError;

    fn try_from(data: Vec<u8>) -> URResult<Self> {
        Ok(Self::new(data))
    }
}

impl TryInto<Vec<u8>> for ZcashBatchSigResult {
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
        let registry = ZcashBatchSigResult::try_from(payload.clone()).unwrap();
        assert_eq!(registry.get_data(), payload);
        assert_eq!(TryInto::<Vec<u8>>::try_into(registry).unwrap(), payload);
    }
}
