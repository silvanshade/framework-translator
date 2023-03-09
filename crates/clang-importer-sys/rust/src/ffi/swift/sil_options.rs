#[cxx::bridge]
pub(crate) mod ffi {
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

use self::ffi::CxxSILOptions;
use cxx::UniquePtr;

pub struct SILOptions {
    pub(crate) ptr: UniquePtr<CxxSILOptions>,
}

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
