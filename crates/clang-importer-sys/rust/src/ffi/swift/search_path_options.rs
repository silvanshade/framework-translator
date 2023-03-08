#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct SearchPathOptions {
        ptr: UniquePtr<CxxSearchPathOptions>,
    }

    #[namespace = "swift"]
    unsafe extern "C++" {
        include!("swift/AST/SearchPathOptions.h");

        #[cxx_name = "SearchPathOptions"]
        type CxxSearchPathOptions;
    }

    #[namespace = "cxx::swift::SearchPathOptions"]
    unsafe extern "C++" {
        include!("cxx/swift/SearchPathOptions.hxx");

        fn make() -> UniquePtr<CxxSearchPathOptions>;
    }
}

use self::ffi::SearchPathOptions;

impl SearchPathOptions {
    #[inline]
    pub fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}

impl Default for SearchPathOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
