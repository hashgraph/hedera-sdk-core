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

use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};
use std::hash::{
    Hash,
    Hasher,
};
use std::str::FromStr;

use hedera_proto::services;
use pkcs8::der::{
    Decode,
    Encode,
};
use prost::Message;
use serde_with::{
    DeserializeFromStr,
    SerializeDisplay,
};

use crate::key::private_key::ED25519_OID;
use crate::{
    Error,
    FromProtobuf,
};

/// A public key on the Hedera network.
#[derive(Clone, Eq, Copy, PartialEq, SerializeDisplay, DeserializeFromStr)]
pub struct PublicKey(PublicKeyData);

#[derive(Clone, Eq, Copy, PartialEq)]
enum PublicKeyData {
    Ed25519(ed25519_dalek::PublicKey),
    EcdsaSecp256k1(k256::ecdsa::VerifyingKey),
}

impl PublicKey {
    pub(super) fn ed25519(key: ed25519_dalek::PublicKey) -> Self {
        Self(PublicKeyData::Ed25519(key))
    }

    pub(super) fn ecdsa_secp256k1(key: k256::ecdsa::VerifyingKey) -> Self {
        Self(PublicKeyData::EcdsaSecp256k1(key))
    }

    #[must_use]
    pub fn is_ed25519(&self) -> bool {
        matches!(&self.0, PublicKeyData::Ed25519(_))
    }

    #[must_use]
    pub fn is_ecdsa_secp256k1(&self) -> bool {
        matches!(&self.0, PublicKeyData::EcdsaSecp256k1(_))
    }

    pub(crate) fn from_alias_bytes(bytes: &[u8]) -> crate::Result<Option<Self>> {
        if !bytes.is_empty() {
            Ok(Some(PublicKey::from_protobuf(
                services::Key::decode(bytes).map_err(|err| Error::from_protobuf(err))?,
            )?))
        } else {
            Ok(None)
        }
    }

    /// Parse a `PublicKey` from a sequence of bytes.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        if bytes.len() == 32 {
            return Self::from_bytes_ed25519(bytes);
        }

        if bytes.len() == 33 {
            return Self::from_bytes_ecdsa_secp256k1(bytes);
        }

        Self::from_bytes_pkcs8_der(bytes)
    }

    /// Parse a Ed25519 `PublicKey` from a sequence of bytes.
    pub fn from_bytes_ed25519(bytes: &[u8]) -> crate::Result<Self> {
        let data = if bytes.len() == 32 {
            ed25519_dalek::PublicKey::from_bytes(bytes).map_err(Error::key_parse)?
        } else {
            return Self::from_bytes_pkcs8_der(bytes);
        };

        Ok(Self::ed25519(data))
    }

    /// Parse a ECDSA(secp256k1) `PublicKey` from a sequence of bytes.
    pub fn from_bytes_ecdsa_secp256k1(bytes: &[u8]) -> crate::Result<Self> {
        let data = if bytes.len() == 33 {
            k256::ecdsa::VerifyingKey::from_sec1_bytes(bytes).map_err(Error::key_parse)?
        } else {
            return Self::from_bytes_pkcs8_der(bytes);
        };

        Ok(Self::ecdsa_secp256k1(data))
    }

    fn from_bytes_pkcs8_der(bytes: &[u8]) -> crate::Result<Self> {
        let info = pkcs8::SubjectPublicKeyInfo::from_der(bytes)
            .map_err(|err| Error::key_parse(err.to_string()))?;

        if info.algorithm.oid == k256::elliptic_curve::ALGORITHM_OID {
            return Self::from_bytes_ecdsa_secp256k1(info.subject_public_key);
        }

        if info.algorithm.oid == ED25519_OID {
            return Self::from_bytes_ed25519(info.subject_public_key);
        }

        Err(Error::key_parse(format!("unsupported key algorithm: {}", info.algorithm.oid)))
    }

    /// Return this private key, serialized as bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(64);

        match &self.0 {
            PublicKeyData::Ed25519(key) => {
                let key = key.to_bytes();
                let info = pkcs8::SubjectPublicKeyInfo {
                    algorithm: self.algorithm(),
                    subject_public_key: &key,
                };

                info.encode_to_vec(&mut buf).unwrap();
            }

            PublicKeyData::EcdsaSecp256k1(key) => {
                let key = key.to_bytes();
                let info = pkcs8::SubjectPublicKeyInfo {
                    algorithm: self.algorithm(),
                    subject_public_key: key.as_slice(),
                };

                info.encode_to_vec(&mut buf).unwrap();
            }
        }

        buf
    }

    fn algorithm(&self) -> pkcs8::AlgorithmIdentifier<'_> {
        pkcs8::AlgorithmIdentifier {
            parameters: None,
            oid: match self.0 {
                PublicKeyData::Ed25519(_) => ED25519_OID,
                PublicKeyData::EcdsaSecp256k1(_) => k256::elliptic_curve::ALGORITHM_OID,
            },
        }
    }

    pub(crate) fn to_bytes_raw(&self) -> Vec<u8> {
        match &self.0 {
            PublicKeyData::Ed25519(key) => key.to_bytes().as_slice().to_vec(),
            PublicKeyData::EcdsaSecp256k1(key) => key.to_bytes().as_slice().to_vec(),
        }
    }
}

impl Debug for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(&hex::encode(self.to_bytes()))
    }
}

impl Hash for PublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            PublicKeyData::Ed25519(key) => key.to_bytes().hash(state),
            PublicKeyData::EcdsaSecp256k1(key) => key.to_bytes().hash(state),
        }
    }
}

impl FromStr for PublicKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(&hex::decode(s).map_err(Error::key_parse)?)
    }
}

impl FromProtobuf<services::Key> for PublicKey {
    fn from_protobuf(pb: services::Key) -> crate::Result<Self>
    where
        Self: Sized,
    {
        use services::key::Key::*;

        match pb.key {
            Some(Ed25519(bytes)) => PublicKey::from_bytes_ed25519(&bytes),
            Some(ContractId(_)) => {
                Err(Error::from_protobuf("unexpected unsupported Contract ID key in single key"))
            }
            Some(DelegatableContractId(_)) => Err(Error::from_protobuf(
                "unexpected unsupported Delegatable Contract ID key in single key",
            )),
            Some(Rsa3072(_)) => {
                Err(Error::from_protobuf("unexpected unsupported RSA-3072 key in single key"))
            }
            Some(Ecdsa384(_)) => {
                Err(Error::from_protobuf("unexpected unsupported ECDSA-384 key in single key"))
            }
            Some(ThresholdKey(_)) => {
                Err(Error::from_protobuf("unexpected threshold key as single key"))
            }
            Some(KeyList(_)) => Err(Error::from_protobuf("unexpected key list as single key")),
            Some(EcdsaSecp256k1(bytes)) => PublicKey::from_bytes_ecdsa_secp256k1(&bytes),
            None => Err(Error::from_protobuf("unexpected empty key in single key")),
        }
    }
}

// TODO: from_protobuf
// TODO: to_protobuf
// TODO: verify_message
// TODO: verify_transaction
