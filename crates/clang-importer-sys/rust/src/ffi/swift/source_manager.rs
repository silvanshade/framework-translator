#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "swift"]
    extern "C++" {
        include!("swift/Basic/SourceManager.h");

        #[cxx_name = "SourceManager"]
        type CxxSourceManager;
    }

    #[namespace = "cxx::swift::SourceManager"]
    extern "C++" {
        include!("cxx/swift/SourceManager.hxx");

        unsafe fn make() -> UniquePtr<CxxSourceManager>;
    }
}

use self::ffi::CxxSourceManager;
use cxx::UniquePtr;

pub struct SourceManager {
    pub(crate) ptr: UniquePtr<CxxSourceManager>,
}

impl From<UniquePtr<CxxSourceManager>> for SourceManager {
    #[inline]
    fn from(ptr: UniquePtr<CxxSourceManager>) -> Self {
        Self { ptr }
    }
}

impl SourceManager {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}
