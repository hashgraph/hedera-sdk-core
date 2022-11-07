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

use std::os::raw::c_char;
use std::ptr;
use std::str::FromStr;

use libc::size_t;

use crate::ffi::error::Error;
use crate::ffi::util::cstr_from_ptr;
use crate::protobuf::ToProtobuf;
use crate::PublicKey;

#[repr(C)]
pub struct AccountId {
    shard: u64,
    realm: u64,
    num: u64,
    /// Safety:
    /// - If `alias` is not null, it must:
    ///   - be properly aligned
    ///   - be dereferenceable
    ///   - point to a valid instance of `PublicKey` (any `PublicKey` that `hedera` provides which hasn't been freed yet)
    alias: *mut PublicKey,
}

impl AccountId {
    // ties the lifetime of `PublicKey` to `self`, which is likely overly restrictive
    pub(super) fn borrow_ref<'a>(&'a self) -> RefAccountId<'a> {
        // safety: invariants of self require a non-null `PublicKey` to follow the required invariants of `NonNull::as_ref`.
        let alias = unsafe { self.alias.as_ref() };

        RefAccountId { shard: self.shard, realm: self.realm, num: self.num, alias }
    }
}

impl From<crate::AccountId> for AccountId {
    fn from(id: crate::AccountId) -> Self {
        Self {
            shard: id.shard,
            realm: id.realm,
            num: id.num,
            alias: id.alias.map(Box::new).map_or_else(ptr::null_mut, Box::into_raw),
        }
    }
}

// sr: why clone when you could just not.
pub(super) struct RefAccountId<'a> {
    shard: u64,
    realm: u64,
    num: u64,
    alias: Option<&'a PublicKey>,
}

impl<'a> RefAccountId<'a> {
    fn into_bytes(self) -> Vec<u8> {
        use prost::Message;
        self.to_protobuf().encode_to_vec()
    }
}

impl<'a> ToProtobuf for RefAccountId<'a> {
    type Protobuf = hedera_proto::services::AccountId;

    fn to_protobuf(&self) -> Self::Protobuf {
        use hedera_proto::services;

        services::AccountId {
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
            account: Some(match self.alias {
                None => services::account_id::Account::AccountNum(self.num as i64),
                Some(alias) => services::account_id::Account::Alias(alias.to_bytes_raw()),
            }),
        }
    }
}

/// Parse a Hedera `AccountId` from the passed string.
#[no_mangle]
pub unsafe extern "C" fn hedera_account_id_from_string(
    s: *const c_char,
    id: *mut AccountId,
) -> Error {
    assert!(!id.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    let parsed = ffi_try!(crate::AccountId::from_str(&s)).into();

    unsafe {
        ptr::write(id, parsed);
    }

    Error::Ok
}

/// Parse a Hedera `AccountId` from the passed bytes.
#[no_mangle]
pub unsafe extern "C" fn hedera_account_id_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    id: *mut AccountId,
) -> Error {
    assert!(!bytes.is_null());
    assert!(!id.is_null());

    let bytes = unsafe { std::slice::from_raw_parts(bytes, bytes_size) };

    let parsed = ffi_try!(crate::AccountId::from_bytes(&bytes)).into();

    unsafe {
        ptr::write(id, parsed);
    }

    Error::Ok
}

/// Serialize the passed `AccountId` as bytes
///
/// # Safety
/// - `id` must uphold the safety requirements of `AccountId`.
/// - `buf` must be valid for writes.
/// - `buf` must only be freed with `hedera_bytes_free`, notably this means that it must not be freed with `free`.
#[no_mangle]
pub unsafe extern "C" fn hedera_account_id_to_bytes(id: AccountId, buf: *mut *mut u8) -> size_t {
    let bytes = id.borrow_ref().into_bytes().into_boxed_slice();

    let bytes = Box::leak(bytes);
    let len = bytes.len();
    let bytes = bytes.as_mut_ptr();

    // safety: invariants promise that `buf` must be valid for writes.
    unsafe {
        ptr::write(buf, bytes);
    }

    len
}
