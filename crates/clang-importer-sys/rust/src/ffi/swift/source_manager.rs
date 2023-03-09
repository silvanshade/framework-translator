#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct SourceManager {
        ptr: UniquePtr<CxxSourceManager>,
    }

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

use self::ffi::{CxxSourceManager, SourceManager};
use cxx::UniquePtr;

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
