#[cxx::bridge]
pub(crate) mod ffi {
    #[derive(Clone)]
    #[namespace = "rust::llvm"]
    struct Twine {
        ptr: SharedPtr<CxxTwine>,
    }

    #[namespace = "llvm"]
    unsafe extern "C++" {
        include!("llvm/ADT/Twine.h");

        #[cxx_name = "Twine"]
        type CxxTwine;
    }

    #[namespace = "cxx::llvm::Twine"]
    unsafe extern "C++" {
        include!("cxx/llvm/Twine.hxx");

        #[namespace = "rust::llvm"]
        type StringRef = crate::llvm::StringRef;

        fn make() -> SharedPtr<CxxTwine>;

        fn from_cxx_string(Str: &CxxString) -> SharedPtr<CxxTwine>;

        fn from_string_ref(Str: &StringRef) -> SharedPtr<CxxTwine>;
    }
}

use self::ffi::Twine;
use crate::llvm::StringRef;
use cxx::CxxString;

impl Twine {
    #[inline]
    pub fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}

impl From<&CxxString> for Twine {
    #[inline]
    fn from(value: &cxx::CxxString) -> Self {
        let ptr = self::ffi::from_cxx_string(value);
        Self { ptr }
    }
}

impl From<&StringRef> for Twine {
    fn from(value: &StringRef) -> Self {
        let ptr = self::ffi::from_string_ref(value);
        Self { ptr }
    }
}
