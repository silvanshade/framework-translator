#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct SILOptions {
        ptr: UniquePtr<CxxSILOptions>,
    }

    #[namespace = "swift"]
    unsafe extern "C++" {
        include!("swift/AST/SILOptions.h");

        #[cxx_name = "SILOptions"]
        type CxxSILOptions;
    }

    #[namespace = "cxx::swift::SILOptions"]
    unsafe extern "C++" {
        include!("cxx/swift/SILOptions.hxx");

        fn make() -> UniquePtr<CxxSILOptions>;
    }
}

use self::ffi::SILOptions;

impl SILOptions {
    #[inline]
    pub fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}
