use ockam_core::Error;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, PartialEq)]
/// Error type relating to FFI specific failures.
pub struct FfiOckamError {
    code: i32,
    domain: *const c_char,
}

impl FfiOckamError {
    /// Create a new error.
    pub fn new(code: i32, domain: &'static str) -> Self {
        Self {
            code,
            domain: domain.as_ptr() as *const c_char,
        }
    }

    /// No error.
    pub fn none() -> Self {
        Self {
            code: 0,
            domain: std::ptr::null(),
        }
    }
}

impl From<Error> for FfiOckamError {
    fn from(err: Error) -> Self {
        Self::new(err.code() as i32, err.domain())
    }
}

/// Represents the failures that can occur in an Ockam FFI Vault.
#[derive(Clone, Copy, Debug)]
pub enum FfiError {
    /// No error.
    None,

    /// Persistence is not supported for this Vault implementation.
    VaultDoesntSupportPersistence,

    /// An underlying filesystem error prevented Vault creation.
    ErrorCreatingFilesystemVault,

    /// Invalid parameter.
    InvalidParam,

    /// Entry not found.
    EntryNotFound,

    /// Unknown public key type.
    UnknownPublicKeyType,

    /// Invalid string.
    InvalidString,

    /// Buffer is too small.
    BufferTooSmall,

    /// A public key is invalid.
    InvalidPublicKey,

    /// No such Vault.
    VaultNotFound,

    /// Ownership error.
    OwnershipError,
}

impl FfiError {
    /// Integer code associated with the error domain.
    pub const DOMAIN_CODE: u32 = 13_000;
    /// Descriptive name for the error domain.
    pub const DOMAIN_NAME: &'static str = "OCKAM_FFI";
}

impl From<FfiError> for Error {
    fn from(err: FfiError) -> Self {
        Self::new(FfiError::DOMAIN_CODE + (err as u32), FfiError::DOMAIN_NAME)
    }
}

impl From<FfiError> for FfiOckamError {
    fn from(err: FfiError) -> Self {
        let err: Error = err.into();
        Self::from(err)
    }
}
