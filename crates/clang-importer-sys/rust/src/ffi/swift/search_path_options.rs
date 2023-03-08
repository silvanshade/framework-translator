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

use self::ffi::SearchPathOptions;

impl SearchPathOptions {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}
