use cosmwasm_std::{Addr, Binary, StdResult, Storage, Uint128};
use k256::ecdsa::{SigningKey, Signature, VerifyingKey};
use rand_core::OsRng;
use sha2::{Sha256, Digest};
use subtle::ConstantTimeEq;

pub struct SecurityModule {
    signing_key: SigningKey,
}

impl SecurityModule {
    pub fn new() -> Self {
        Self {
            signing_key: SigningKey::random(&mut OsRng),
        }
    }

    pub fn sign_transaction(&self, msg: &[u8]) -> Signature {
        self.signing_key.sign(msg)
    }

    pub fn verify_signature(&self, msg: &[u8], signature: &Signature) -> bool {
        let verifying_key = VerifyingKey::from(&self.signing_key);
        verifying_key.verify(msg, signature).is_ok()
    }

    pub fn secure_hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        a.ct_eq(b).into()
    }
}
