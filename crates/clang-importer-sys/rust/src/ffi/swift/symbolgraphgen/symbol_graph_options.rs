#[cxx::bridge(namespace = "rust::swift::symbolgraphgen")]
pub(crate) mod ffi {
    struct SymbolGraphOptions {
        ptr: UniquePtr<CxxSymbolGraphOptions>,
    }

    #[namespace = "swift::symbolgraphgen"]
    extern "C++" {
        include!("swift/SymbolGraphGen/SymbolGraphOptions.h");

        #[cxx_name = "SymbolGraphOptions"]
        type CxxSymbolGraphOptions;
    }

    #[namespace = "cxx::swift::symbolgraphgen::SymbolGraphOptions"]
    extern "C++" {
        include!("cxx/swift/symbolgraphgen/SymbolGraphOptions.hxx");

        unsafe fn make() -> UniquePtr<CxxSymbolGraphOptions>;
    }
}

use self::ffi::SymbolGraphOptions;

impl SymbolGraphOptions {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}
