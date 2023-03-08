#[cxx::bridge]
pub(crate) mod ffi {
    #[derive(Clone)]
    #[namespace = "rust::llvm"]
    struct Twine {
        ptr: SharedPtr<CxxTwine>,
    }

    #[namespace = "llvm"]
    extern "C++" {
        include!("llvm/ADT/Twine.h");

        #[cxx_name = "Twine"]
        type CxxTwine;
    }

    #[namespace = "cxx::llvm::Twine"]
    extern "C++" {
        include!("cxx/llvm/Twine.hxx");

        #[namespace = "rust::llvm"]
        type StringRef = crate::llvm::StringRef;

        unsafe fn make() -> SharedPtr<CxxTwine>;

        unsafe fn from_cxx_string(str: &CxxString) -> SharedPtr<CxxTwine>;

        unsafe fn from_string_ref(str: &StringRef) -> SharedPtr<CxxTwine>;
    }
}

use self::ffi::Twine;
use crate::llvm::StringRef;

impl Twine {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }

    #[inline]
    pub unsafe fn from_cxx_string(str: &cxx::CxxString) -> Self {
        let ptr = self::ffi::from_cxx_string(str);
        Self { ptr }
    }

    #[inline]
    pub unsafe fn from_string_ref(str: &StringRef) -> Self {
        let ptr = self::ffi::from_string_ref(str);
        Self { ptr }
    }
}
