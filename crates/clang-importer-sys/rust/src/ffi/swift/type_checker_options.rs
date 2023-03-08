#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct TypeCheckerOptions {
        ptr: UniquePtr<CxxTypeCheckerOptions>,
    }

    #[namespace = "swift"]
    extern "C++" {
        include!("swift/Basic/LangOptions.h");

        #[cxx_name = "TypeCheckerOptions"]
        type CxxTypeCheckerOptions;
    }

    #[namespace = "cxx::swift::TypeCheckerOptions"]
    extern "C++" {
        include!("cxx/swift/TypeCheckerOptions.hxx");

        unsafe fn make() -> UniquePtr<CxxTypeCheckerOptions>;
    }
}

use self::ffi::TypeCheckerOptions;

impl TypeCheckerOptions {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}
