#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct SearchPathOptions {
        ptr: UniquePtr<CxxSearchPathOptions>,
    }

    #[namespace = "swift"]
    extern "C++" {
        include!("swift/AST/SearchPathOptions.h");

        #[cxx_name = "SearchPathOptions"]
        type CxxSearchPathOptions;
    }

    #[namespace = "cxx::swift::SearchPathOptions"]
    extern "C++" {
        include!("cxx/swift/SearchPathOptions.hxx");

        unsafe fn make() -> UniquePtr<CxxSearchPathOptions>;
    }
}

use self::ffi::{CxxSearchPathOptions, SearchPathOptions};
use cxx::UniquePtr;

impl From<UniquePtr<CxxSearchPathOptions>> for SearchPathOptions {
    #[inline]
    fn from(ptr: UniquePtr<CxxSearchPathOptions>) -> Self {
        Self { ptr }
    }
}

impl SearchPathOptions {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}
