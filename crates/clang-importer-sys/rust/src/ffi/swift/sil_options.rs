#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct SILOptions {
        ptr: UniquePtr<CxxSILOptions>,
    }

    #[namespace = "swift"]
    extern "C++" {
        include!("swift/AST/SILOptions.h");

        #[cxx_name = "SILOptions"]
        type CxxSILOptions;
    }

    #[namespace = "cxx::swift::SILOptions"]
    extern "C++" {
        include!("cxx/swift/SILOptions.hxx");

        unsafe fn make() -> UniquePtr<CxxSILOptions>;
    }
}

use self::ffi::{CxxSILOptions, SILOptions};
use cxx::UniquePtr;

impl From<UniquePtr<CxxSILOptions>> for SILOptions {
    #[inline]
    fn from(ptr: UniquePtr<CxxSILOptions>) -> Self {
        Self { ptr }
    }
}

impl SILOptions {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}
