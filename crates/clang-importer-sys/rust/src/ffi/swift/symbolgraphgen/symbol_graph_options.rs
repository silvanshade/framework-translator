#[cxx::bridge]
pub(crate) mod ffi {
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

use self::ffi::CxxSymbolGraphOptions;
use cxx::UniquePtr;

pub struct SymbolGraphOptions {
    pub(crate) ptr: UniquePtr<CxxSymbolGraphOptions>,
}

impl From<UniquePtr<CxxSymbolGraphOptions>> for SymbolGraphOptions {
    #[inline]
    fn from(ptr: UniquePtr<CxxSymbolGraphOptions>) -> Self {
        Self { ptr }
    }
}

impl SymbolGraphOptions {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}
