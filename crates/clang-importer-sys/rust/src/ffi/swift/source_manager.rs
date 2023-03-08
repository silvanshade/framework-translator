#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct SourceManager {
        ptr: UniquePtr<CxxSourceManager>,
    }

    #[namespace = "swift"]
    unsafe extern "C++" {
        include!("swift/Basic/SourceManager.h");

        #[cxx_name = "SourceManager"]
        type CxxSourceManager;
    }

    #[namespace = "cxx::swift::SourceManager"]
    unsafe extern "C++" {
        include!("cxx/swift/SourceManager.hxx");

        fn make() -> UniquePtr<CxxSourceManager>;
    }
}

use self::ffi::SourceManager;

impl SourceManager {
    #[inline]
    pub fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}

impl Default for SourceManager {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
