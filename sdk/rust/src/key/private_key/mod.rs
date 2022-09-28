/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

#[cfg(test)]
mod tests;

use std::fmt::{
    Debug,
    Display,
    Formatter,
};
use std::str::FromStr;
use std::sync::Arc;
use std::{
    fmt,
    iter,
};

use ed25519_dalek::{
    Keypair,
    Signer,
};
use hmac::{
    Hmac,
    Mac,
};
use k256::ecdsa::signature::DigestSigner;
use k256::pkcs8::der::Encode;
use pkcs8::der::Decode;
use pkcs8::{
    AssociatedOid,
    ObjectIdentifier,
};
use rand::{
    thread_rng,
    Rng,
};
use sha2::Sha512;
use sha3::Digest;

use crate::{
    AccountId,
    Error,
    PublicKey,
    SignaturePair,
};

pub(super) const ED25519_OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.3.101.112");

/// A private key on the Hedera network.
#[derive(Clone)]
pub struct PrivateKey(Arc<PrivateKeyDataWrapper>);

// find a better name
struct PrivateKeyDataWrapper {
    data: PrivateKeyData,
    chain_code: Option<[u8; 32]>,
}

impl PrivateKeyDataWrapper {
    fn new(inner: PrivateKeyData) -> Self {
        Self { data: inner, chain_code: None }
    }

    fn new_derivable(inner: PrivateKeyData, chain_code: [u8; 32]) -> Self {
        Self { data: inner, chain_code: Some(chain_code) }
    }
}

// for usage in tests (provides a way to snapshot test)
impl Debug for PrivateKeyDataWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        #[derive(Debug)]
        enum Algorithm {
            Ed25519,
            Ecdsa,
        }

        let (algorithm, key) = match &self.data {
            PrivateKeyData::Ed25519(key) => {
                (Algorithm::Ed25519, hex::encode(key.secret.as_bytes()))
            }

            PrivateKeyData::Ecdsa(key) => (Algorithm::Ecdsa, hex::encode(&key.to_bytes())),
        };

        f.debug_struct("PrivateKeyData")
            .field("algorithm", &algorithm)
            .field("key", &key)
            .field("chain_code", &self.chain_code.as_ref().map(hex::encode))
            .finish()
    }
}

enum PrivateKeyData {
    Ed25519(ed25519_dalek::Keypair),
    Ecdsa(k256::ecdsa::SigningKey),
}

impl PrivateKey {
    #[cfg(test)]
    pub(crate) fn debug_pretty(&self) -> &impl Debug {
        &*self.0
    }

    /// Generates a new Ed25519 `PrivateKey`.
    #[must_use]
    pub fn generate_ed25519() -> Self {
        let mut csprng = thread_rng();
        let data = ed25519_dalek::Keypair::generate(&mut csprng);
        let data = PrivateKeyData::Ed25519(data);

        let mut chain_code = [0u8; 32];
        csprng.fill(&mut chain_code);

        Self(Arc::new(PrivateKeyDataWrapper::new_derivable(data, chain_code)))
    }

    /// Generates a new ECDSA(secp256k1) `PrivateKey`.
    #[must_use]
    pub fn generate_ecdsa() -> Self {
        let data = k256::ecdsa::SigningKey::random(&mut thread_rng());
        let data = PrivateKeyData::Ecdsa(data);

        Self(Arc::new(PrivateKeyDataWrapper::new(data)))
    }

    /// Gets the [`PublicKey`] which corresponds to this `PrivateKey`.
    #[must_use]
    pub fn public_key(&self) -> PublicKey {
        match &self.0.data {
            PrivateKeyData::Ed25519(key) => PublicKey::ed25519(key.public),
            PrivateKeyData::Ecdsa(key) => PublicKey::ecdsa(key.verifying_key()),
        }
    }

    /// Parse a `PrivateKey` from a sequence of bytes.
    ///
    /// # Errors
    /// - [`Error::KeyParse`] if `bytes` cannot be parsed into a `PublicKey`.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        if bytes.len() == 32 || bytes.len() == 64 {
            return Self::from_bytes_ed25519(bytes);
        }

        Self::from_bytes_der(bytes)
    }

    /// Parse a Ed25519 `PrivateKey` from a sequence of bytes.
    ///
    /// # Errors
    /// - [`Error::KeyParse`] if `bytes` cannot be parsed into a ed25519 `PrivateKey`.
    pub fn from_bytes_ed25519(bytes: &[u8]) -> crate::Result<Self> {
        let data = if bytes.len() == 32 || bytes.len() == 64 {
            ed25519_dalek::SecretKey::from_bytes(&bytes[..32]).map_err(Error::key_parse)?
        } else {
            return Self::from_bytes_der(bytes);
        };

        let data = Keypair { public: (&data).into(), secret: data };

        Ok(Self(Arc::new(PrivateKeyDataWrapper::new(PrivateKeyData::Ed25519(data)))))
    }

    /// Parse a ECDSA(secp256k1) `PrivateKey` from a sequence of bytes.
    ///
    /// # Errors
    /// - [`Error::KeyParse`] if `bytes` cannot be parsed into a ECDSA(secp256k1) `PrivateKey`.
    pub fn from_bytes_ecdsa(bytes: &[u8]) -> crate::Result<Self> {
        let data = if bytes.len() == 32 {
            // not DER encoded, raw bytes for key
            k256::ecdsa::SigningKey::from_bytes(bytes).map_err(Error::key_parse)?
        } else {
            return Self::from_bytes_der(bytes);
        };

        Ok(Self(Arc::new(PrivateKeyDataWrapper::new(PrivateKeyData::Ecdsa(data)))))
    }

    /// Parse a `PrivateKey` from a sequence of der encoded bytes.
    ///
    /// # Errors
    /// - [`Error::KeyParse`] if `bytes` cannot be parsed into a `PrivateKey`.
    pub fn from_bytes_der(bytes: &[u8]) -> crate::Result<Self> {
        let info = pkcs8::PrivateKeyInfo::from_der(bytes)
            .map_err(|err| Error::key_parse(err.to_string()))?;

        // PrivateKey is an `OctetString`, and the `PrivateKey`s we all support are `OctetStrings`.
        // So, we, awkwardly, have an `OctetString` containing an `OctetString` containing our key material.
        let inner = pkcs8::der::asn1::OctetStringRef::from_der(info.private_key)
            .map_err(|err| Error::key_parse(err.to_string()))?;

        let inner = inner.as_bytes();

        if info.algorithm.oid == k256::Secp256k1::OID {
            return Self::from_bytes_ecdsa(inner);
        }

        if info.algorithm.oid == ED25519_OID {
            return Self::from_bytes_ed25519(inner);
        }

        Err(Error::key_parse(format!("unsupported key algorithm: {}", info.algorithm.oid)))
    }

    /// Parse a `PrivateKey` from a der encoded string.
    ///
    /// Optionally strips a `0x` prefix.
    /// See [`from_bytes_der`](self::from_bytes_der).
    ///
    /// # Errors
    /// - [`Error::KeyParse`] if `s` cannot be parsed into a `PrivateKey`.
    pub fn from_str_der(s: &str) -> crate::Result<Self> {
        Self::from_bytes_der(
            &hex::decode(s.strip_prefix("0x").unwrap_or(s)).map_err(Error::key_parse)?,
        )
    }

    /// Parse a Ed25519 `PrivateKey` from a string containing the raw key material.
    ///
    /// Optionally strips a `0x` prefix.
    /// See: [`from_bytes_ed25519`](Self::from_bytes_ed25519).
    ///
    /// # Errors
    /// - [`Error::KeyParse`] if `s` cannot be parsed into a ed25519 `PrivateKey`.
    pub fn from_str_ed25519(s: &str) -> crate::Result<Self> {
        Self::from_bytes_ed25519(
            &hex::decode(s.strip_prefix("0x").unwrap_or(s)).map_err(Error::key_parse)?,
        )
    }

    /// Parse a ECDSA(secp256k1) `PrivateKey` from a string containing the raw key material.
    ///
    /// Optionally strips a `0x` prefix.
    /// See: [`frobytestr_ecdsa`](Self::frobytestr_ecdsa).
    ///
    /// # Errors
    /// - [`Error::KeyParse`] if `s` cannot be parsed into a ECDSA(secp256k1) `PrivateKey`.
    pub fn from_str_ecdsa(s: &str) -> crate::Result<Self> {
        Self::from_bytes_ecdsa(
            &hex::decode(s.strip_prefix("0x").unwrap_or(s)).map_err(Error::key_parse)?,
        )
    }

    /// Parse a `PrivateKey` from [PEM](https://www.rfc-editor.org/rfc/rfc7468#section-10) encoded bytes.
    ///
    /// # Errors
    /// - [`Error::KeyParse`] if `pem` is not valid PEM.
    /// - [`Error::KeyParse`] if the type label (BEGIN XYZ) is not `PRIVATE KEY`.
    /// - [`Error::KeyParse`] if the data contained inside the PEM is not a valid `PrivateKey`.
    pub fn from_pem(pem: &[u8]) -> crate::Result<Self> {
        let (type_label, der) = pem_rfc7468::decode_vec(pem).map_err(Error::key_parse)?;

        if type_label != "PRIVATE KEY" {
            return Err(Error::key_parse(format!(
                "incorrect PEM type label: expected: `PRIVATE KEY`, got: `{type_label}`"
            )));
        }

        Self::from_bytes_der(&der)
    }

    /// Return this `PrivateKey`, serialized as bytes.
    // panic should be impossible (`unreachable`)
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn to_bytes_der(&self) -> Vec<u8> {
        let mut inner = Vec::with_capacity(34);

        pkcs8::der::asn1::OctetStringRef::new(&self.to_bytes_raw())
            .unwrap()
            .encode_to_vec(&mut inner)
            .unwrap();

        let info = pkcs8::PrivateKeyInfo {
            algorithm: self.algorithm(),
            private_key: &inner,
            public_key: None,
        };

        let mut buf = Vec::with_capacity(64);
        info.encode_to_vec(&mut buf).unwrap();

        buf
    }

    /// Return this `PrivateKey`, serialized as bytes.
    ///
    /// If this is an ed25519 private key, this is equivalent to [`to_bytes_raw`](Self::to_bytes_raw)
    /// If this is an ecdsa private key, this is equivalent to [`to_bytes_der`](Self::to_bytes_der)
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        match &self.0.data {
            PrivateKeyData::Ed25519(_) => self.to_bytes_raw().as_slice().to_vec(),
            PrivateKeyData::Ecdsa(_) => self.to_bytes_der(),
        }
    }

    fn to_bytes_raw(&self) -> [u8; 32] {
        match &self.0.data {
            PrivateKeyData::Ed25519(key) => key.secret.to_bytes(),
            PrivateKeyData::Ecdsa(key) => key.to_bytes().into(),
        }
    }

    /// DER encodes self, then hex encodes the result.
    #[must_use]
    pub fn to_string_der(&self) -> String {
        hex::encode(self.to_bytes_der())
    }

    /// Returns the raw bytes of `self` after hex encoding.
    #[must_use]
    pub fn to_string_raw(&self) -> String {
        hex::encode(self.to_bytes_raw())
    }

    /// Creates an [`AccountId`] with the given `shard`, `realm`, and `self.public_key()` as an [`alias`](AccountId::alias).
    ///
    /// # Examples
    ///
    /// FIXME: this is 100% broken (but it's not this function's fault).
    /// ```,no_run
    /// use hedera::PublicKey;
    ///
    /// let key: PublicKey = "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7".parse().unwrap();
    ///
    /// let account_id = key.to_account_id(0, 0);
    /// assert_eq!(account_id.to_string(), "0.0.<todo>");
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn to_account_id(&self, shard: u64, realm: u64) -> AccountId {
        self.public_key().to_account_id(shard, realm)
    }

    fn algorithm(&self) -> pkcs8::AlgorithmIdentifier<'_> {
        pkcs8::AlgorithmIdentifier {
            parameters: None,
            oid: match &self.0.data {
                PrivateKeyData::Ed25519(_) => ED25519_OID,
                PrivateKeyData::Ecdsa(_) => k256::Secp256k1::OID,
            },
        }
    }

    pub(crate) fn sign(&self, message: &[u8]) -> SignaturePair {
        let public = self.public_key();

        match &self.0.data {
            PrivateKeyData::Ed25519(key) => SignaturePair::ed25519(key.sign(message), public),
            PrivateKeyData::Ecdsa(key) => SignaturePair::ecdsa(
                key.sign_digest(sha3::Keccak256::new_with_prefix(message)),
                public,
            ),
        }
    }

    /// Derives a child key based on `index`.
    ///
    /// # Errors
    /// todo: (what error variant) if this is an Ecdsa key (unsupported operation)
    /// todo: (what error variant) if this key has no `chain_code` (key is not derivable)
    pub fn derive(&self, index: i32) -> crate::Result<Self> {
        const HARDEND_MASK: u32 = 1 << 31;
        let index = index as u32;

        let chain_code = match &self.0.chain_code {
            Some(chain_code) => chain_code,
            // Key is not derivable
            None => todo!(),
        };

        match &self.0.data {
            PrivateKeyData::Ed25519(key) => {
                // force hardened.
                let index = index | HARDEND_MASK;

                let output: [u8; 64] = Hmac::<Sha512>::new_from_slice(chain_code)
                    .expect("HMAC can take keys of any size")
                    .chain_update([0])
                    .chain_update(key.secret.as_bytes())
                    .chain_update(index.to_be_bytes())
                    .finalize()
                    .into_bytes()
                    .into();

                // todo: use `split_array_ref` when that's stable.
                let (left, right) = output.split_at(32);

                // this is exactly 32 bytes
                let chain_code: [u8; 32] = right.try_into().unwrap();

                let data = ed25519_dalek::SecretKey::from_bytes(left).unwrap();
                let data = Keypair { public: (&data).into(), secret: data };
                let data = PrivateKeyData::Ed25519(data);

                Ok(Self(Arc::new(PrivateKeyDataWrapper::new_derivable(data, chain_code))))
            }
            PrivateKeyData::Ecdsa(_) => todo!(),
        }
    }

    // todo: what do we do about i32?
    // It's basically just a cast to support them, but, unlike Java, operator overloading doesn't exist.
    /// Derive a `PrivateKey` based on the `index`.
    ///
    /// # Errors
    /// - <todo: what error variant> if this is an Ecdsa key (unsupported operation)
    // ⚠️ unaudited cryptography ⚠️
    pub fn legacy_derive(&self, index: i64) -> crate::Result<Self> {
        match &self.0.data {
            PrivateKeyData::Ed25519(key) => {
                let entropy = key.secret.as_bytes();
                let mut seed = Vec::with_capacity(entropy.len() + 8);

                seed.extend_from_slice(entropy);

                let i1: i32 = match index {
                    0x00ff_ffff_ffff => 0xff,
                    0.. => 0,
                    _ => -1,
                };

                let i2 = index as u8;

                seed.extend_from_slice(&i1.to_be_bytes());
                // any better way to do this?
                seed.extend(iter::repeat(i2).take(4));

                let salt: Vec<u8> = vec![0xff];

                let mut mat = [0; 32];

                pbkdf2::pbkdf2::<Hmac<Sha512>>(&seed, &salt, 2048, &mut mat);

                // note: this shouldn't fail, but there isn't an infaliable conversion.
                Self::from_bytes_ed25519(&mat)
            }

            // need to add an error variant, key derivation doesn't exist for Ecdsa keys in Java impl.
            PrivateKeyData::Ecdsa(_) => todo!(),
        }
    }

    /// Recover a `PrivateKey` from a generated mnemonic phrase and a passphrase.
    // this is specifically for the two `try_into`s which depend on `split_array_ref`.
    // There *is* a 3rd unwrap for a "key is not derivable" error, but we construct a key that _is_ derivable.
    // Any panic would indicate a bug in this crate or a dependency of it, not in user code.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn from_mnemonic(mnemonic: &crate::Mnemonic, passphrase: &str) -> PrivateKey {
        let seed = mnemonic.to_seed(passphrase);

        let output: [u8; 64] = Hmac::<Sha512>::new_from_slice(b"ed25519 seed")
            .expect("hmac can take a seed of any size")
            .chain_update(&seed)
            .finalize()
            .into_bytes()
            .into();

        // todo: use `split_array_ref` when that's stable.
        let (left, right) = {
            let (left, right) = output.split_at(32);
            let left: [u8; 32] = left.try_into().unwrap();
            let right: [u8; 32] = right.try_into().unwrap();
            (left, right)
        };

        let data = ed25519_dalek::SecretKey::from_bytes(&left).unwrap();
        let data = ed25519_dalek::Keypair { public: (&data).into(), secret: data };
        let data = PrivateKeyData::Ed25519(data);

        let mut key = Self(Arc::new(PrivateKeyDataWrapper::new_derivable(data, right)));

        for index in [44, 3030, 0, 0] {
            key = key.derive(index).expect("BUG: we set the chain code earlier in this function");
        }

        key
    }
}

impl Debug for PrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(&self.to_string_der())
    }
}

impl FromStr for PrivateKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(&hex::decode(s.strip_prefix("0x").unwrap_or(s)).map_err(Error::key_parse)?)
    }
}

// TODO: derive (!) - secp256k1
// TODO: legacy_derive (!) - secp256k1
// TODO: sign_transaction
