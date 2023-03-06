#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct LangOptions {
        ptr: UniquePtr<CxxLangOptions>,
    }

    #[namespace = "swift"]
    unsafe extern "C++" {
        include!("swift/Basic/LangOptions.h");

        #[cxx_name = "LangOptions"]
        type CxxLangOptions;
    }

    #[namespace = "cxx::swift::LangOptions"]
    unsafe extern "C++" {
        include!("cxx/swift/LangOptions.hxx");

        fn make() -> UniquePtr<CxxLangOptions>;
    }
}

use self::ffi::LangOptions;

impl LangOptions {
    #[inline]
    pub fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}
