//! High level Rust wrapper for RPS API.

use std::ptr;

pub mod runtime;

use thiserror::Error;

use rps_sys as ffi;

/// Helper macro to FFI API and map result to `Error` type.
#[macro_export]
macro_rules! call {
    ($f:expr) => {{
        let code = unsafe { $f };
        match code {
            0 => Ok(()),
            _ => Err(Error::from(code)),
        }
    }};
}

/// Error codes used by operations of the RPS library.
#[derive(Debug, Error, Copy, Clone)]
pub enum Error {
    #[error("Unspecified error")]
    Unspecified,

    /// Failure due to an unrecognized command.
    #[error("Unrecognized command")]
    UnrecognizedCommand,

    /// Failure due to invalid arguments.
    #[error("Invalid arguments")]
    InvalidArguments,

    /// Failure due to invalid data.
    #[error("Invalid data")]
    InvalidData,

    /// Failure due to an invalid operation.
    #[error("Invalid operation")]
    InvalidOperation,

    /// Failure due to running out of memory.
    #[error("Out of memory")]
    OutOfMemory,

    /// Failure due to not being able to find the specified file.
    #[error("File not found")]
    FileNotFound,

    /// Failure due to an invalid file format.
    #[error("Invalid file format")]
    InvalidFileFormat,

    /// Failure due to the file format version being too old.
    #[error("File format version too old")]
    UnsupportedVersionTooOld,

    /// Failure due to the file format version being too new.
    #[error("File format version too new")]
    UnsupportedVersionTooNew,

    /// Failure due to an unknown node.
    #[error("Unknown node")]
    UnknownNode,

    /// Failure due to an index being out of its valid bounds.
    #[error("Index out of bounds")]
    IndexOutOfBounds,

    /// Failure due to a command being already finalized.
    #[error("Command already finalized")]
    CommandAlreadyFinal,

    /// Failure due to a data layout mismatch between runtime and shader.
    #[error("Data layout mismatch between runtime and shader")]
    InteropDataLayoutMismatch,

    /// Failure due to a key not being found.
    #[error("Key not found")]
    KeyNotFound,

    /// Failure due to a key value being duplicated where it is required to be unique.
    #[error("Key duplicated")]
    KeyDuplicated,

    /// Failure due to a feature not being implemented yet.
    #[error("Not implemented")]
    NotImplemented,

    /// Failure due to an integer overflow.
    #[error("Integer overflow")]
    IntegerOverflow,

    /// Failure due to exclusive ranges overlapping.
    #[error("Exclusive ranges overlapping")]
    RangeOverlapping,

    /// Failure due to rpsRenderPipelineValidate finding an invalid pipeline configuration. More details are provided
    /// via output of the device print function.
    #[error("Invalid pipeline configuration")]
    ValidationFailed,

    /// Failure due to a compiled RPSL shader program being ill formed. Normally indicates a compiler error.
    #[error("Compiler error")]
    InvalidProgram,

    /// Failure due to an RPSL module being incompatible with the current runtime.
    #[error("RPSL module is incompatible with the current runtime")]
    UnsupportedModuleVersion,

    /// Failure due to a failed type safety check.
    #[error("Type safety check failed")]
    TypeMismatch,

    /// Failure due to a feature not being supported.
    #[error("Not supported")]
    NotSupported,

    /// Failure due to failed a runtime API without direct mapping of the API error code.
    #[error("Runtime API error")]
    RuntimeApiError,

    /// Failure due to an RPS library internal error.
    #[error("RPS library internal error")]
    InternalError,

    #[error("Unmapped error code: {0}")]
    Unknown(ffi::RpsResult),
}

impl From<ffi::RpsResult> for Error {
    fn from(value: ffi::RpsResult) -> Self {
        match value {
            ffi::RpsResult_RPS_ERROR_UNSPECIFIED => Error::Unspecified,
            ffi::RpsResult_RPS_ERROR_UNRECOGNIZED_COMMAND => Error::UnrecognizedCommand,
            ffi::RpsResult_RPS_ERROR_INVALID_ARGUMENTS => Error::InvalidArguments,
            ffi::RpsResult_RPS_ERROR_INVALID_DATA => Error::InvalidData,
            ffi::RpsResult_RPS_ERROR_INVALID_OPERATION => Error::InvalidOperation,
            ffi::RpsResult_RPS_ERROR_OUT_OF_MEMORY => Error::OutOfMemory,
            ffi::RpsResult_RPS_ERROR_FILE_NOT_FOUND => Error::FileNotFound,
            ffi::RpsResult_RPS_ERROR_INVALID_FILE_FORMAT => Error::InvalidFileFormat,
            ffi::RpsResult_RPS_ERROR_UNSUPPORTED_VERSION_TOO_OLD => Error::UnsupportedVersionTooOld,
            ffi::RpsResult_RPS_ERROR_UNSUPPORTED_VERSION_TOO_NEW => Error::UnsupportedVersionTooNew,
            ffi::RpsResult_RPS_ERROR_UNKNOWN_NODE => Error::UnknownNode,
            ffi::RpsResult_RPS_ERROR_INDEX_OUT_OF_BOUNDS => Error::IndexOutOfBounds,
            ffi::RpsResult_RPS_ERROR_COMMAND_ALREADY_FINAL => Error::CommandAlreadyFinal,
            ffi::RpsResult_RPS_ERROR_INTEROP_DATA_LAYOUT_MISMATCH => {
                Error::InteropDataLayoutMismatch
            }
            ffi::RpsResult_RPS_ERROR_KEY_NOT_FOUND => Error::KeyNotFound,
            ffi::RpsResult_RPS_ERROR_KEY_DUPLICATED => Error::KeyDuplicated,
            ffi::RpsResult_RPS_ERROR_NOT_IMPLEMENTED => Error::NotImplemented,
            ffi::RpsResult_RPS_ERROR_INTEGER_OVERFLOW => Error::IntegerOverflow,
            ffi::RpsResult_RPS_ERROR_RANGE_OVERLAPPING => Error::RangeOverlapping,
            ffi::RpsResult_RPS_ERROR_VALIDATION_FAILED => Error::ValidationFailed,
            ffi::RpsResult_RPS_ERROR_INVALID_PROGRAM => Error::InvalidProgram,
            ffi::RpsResult_RPS_ERROR_UNSUPPORTED_MODULE_VERSION => Error::UnsupportedModuleVersion,
            ffi::RpsResult_RPS_ERROR_TYPE_MISMATCH => Error::TypeMismatch,
            ffi::RpsResult_RPS_ERROR_NOT_SUPPORTED => Error::NotSupported,
            ffi::RpsResult_RPS_ERROR_RUNTIME_API_ERROR => Error::RuntimeApiError,
            ffi::RpsResult_RPS_ERROR_INTERNAL_ERROR => Error::InternalError,
            _ => Error::Unknown(value),
        }
    }
}

/// Helper structure to build `ffi::RpsDeviceCreateInfo` for a new device.
#[derive(Default)]
pub struct DeviceBuilder {
    create_info: ffi::RpsDeviceCreateInfo,
}

impl DeviceBuilder {
    /// Create a new device builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify requirements for a single memory allocation.
    ///
    /// # Arguments
    ///
    /// * `size` - Size of the allocation in bytes.
    /// * `alignment` - Minimum alignment requirement of the allocation in bytes.
    pub fn data_alloc_info(&mut self, size: usize, alignment: usize) -> &mut Self {
        self.create_info.privateDataAllocInfo = ffi::RpsAllocInfo { size, alignment };
        self
    }

    /// Creates a device object.
    pub fn build(&self) -> Result<Device, Error> {
        let mut handle = ptr::null_mut();

        call!(ffi::rpsDeviceCreate(&self.create_info, &mut handle))?;

        Ok(Device {
            handle,
            callbacks: None,
        })
    }

    /// Create a device with a dummy runtime.
    pub fn build_null(&self, callbacks: Box<dyn runtime::Callbacks>) -> Result<Device, Error> {
        let mut device = self.build()?;

        let runtime_info = ffi::RpsRuntimeDeviceCreateInfo {
            pUserContext: callbacks.as_ref() as *const _ as *mut _,
            callbacks: runtime::CALLBACKS,
        };

        let create_info = ffi::RpsNullRuntimeDeviceCreateInfo {
            pDeviceCreateInfo: &self.create_info,
            pRuntimeCreateInfo: &runtime_info,
        };

        call!(ffi::rpsNullRuntimeDeviceCreate(
            &create_info,
            &mut device.handle
        ))?;

        device.callbacks = Some(callbacks);

        Ok(device)
    }
}

/// RPS device object.
///
/// The RPS device is used as the main state object for the RPS runtime API. It provides a central location for data
/// and callbacks of the rest of the software stack.
pub struct Device {
    /// Raw handle.
    handle: ffi::RpsDevice,
    /// Runtime callbacks (keep alive until the device is destroyed).
    callbacks: Option<Box<dyn runtime::Callbacks>>,
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { ffi::rpsDeviceDestroy(self.handle) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Dummy;
    impl runtime::Callbacks for Dummy {}

    #[test]
    fn default_device() {
        let device = DeviceBuilder::default().build().unwrap();
        drop(device);
    }

    #[test]
    fn null_runtime() {
        let device = DeviceBuilder::new().build_null(Box::new(Dummy)).unwrap();
        drop(device);
    }
}
