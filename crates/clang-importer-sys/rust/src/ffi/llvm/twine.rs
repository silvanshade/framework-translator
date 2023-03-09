#[cxx::bridge]
pub(crate) mod ffi {
    #[derive(Clone)]
    #[namespace = "rust::llvm"]
    struct Twine<'a> {
        ptr: SharedPtr<CxxTwine<'a>>,
    }

    #[namespace = "llvm"]
    extern "C++" {
        include!("llvm/ADT/Twine.h");

        #[cxx_name = "Twine"]
        type CxxTwine<'a>;
    }

    #[namespace = "cxx::llvm::Twine"]
    unsafe extern "C++" {
        include!("cxx/llvm/Twine.hxx");

        #[namespace = "rust::llvm"]
        type StringRef<'a> = crate::llvm::StringRef<'a>;

        unsafe fn make() -> SharedPtr<CxxTwine<'static>>;

        unsafe fn from_cxx_string<'a>(str: &'a CxxString) -> SharedPtr<CxxTwine<'a>>;

        unsafe fn from_string_ref<'a>(str: &StringRef<'a>) -> SharedPtr<CxxTwine<'a>>;
    }
}

use self::ffi::Twine;
use crate::llvm::StringRef;
use cxx::CxxString;

impl<'a> Twine<'a> {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }

    #[inline]
    pub unsafe fn from_cxx_string(str: &'a CxxString) -> Self {
        let ptr = self::ffi::from_cxx_string(str);
        Self { ptr }
    }

    #[inline]
    pub unsafe fn from_string_ref(str: &StringRef<'a>) -> Self {
        let ptr = self::ffi::from_string_ref(str);
        Self { ptr }
    }
}
