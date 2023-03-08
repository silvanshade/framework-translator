#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct ClangImporterOptions {
        ptr: UniquePtr<CxxClangImporterOptions>,
    }

    #[namespace = "swift"]
    extern "C++" {
        include!("swift/Basic/LangOptions.h");

        #[cxx_name = "ClangImporterOptions"]
        type CxxClangImporterOptions;
    }

    #[namespace = "cxx::swift::ClangImporterOptions"]
    extern "C++" {
        include!("cxx/swift/ClangImporterOptions.hxx");

        unsafe fn make() -> UniquePtr<CxxClangImporterOptions>;
    }
}

use self::ffi::ClangImporterOptions;

impl ClangImporterOptions {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}
