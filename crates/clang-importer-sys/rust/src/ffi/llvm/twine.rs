#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "llvm"]
    extern "C++" {
        include!("llvm/ADT/Twine.h");

        #[cxx_name = "Twine"]
        type CxxTwine<'a>;
    }

    #[namespace = "cxx::llvm::Twine"]
    unsafe extern "C++" {
        include!("cxx/llvm/Twine.hxx");

        #[namespace = "llvm"]
        #[cxx_name = "StringRef"]
        type CxxStringRef<'a> = crate::ffi::llvm::string_ref::ffi::CxxStringRef<'a>;

        unsafe fn make() -> SharedPtr<CxxTwine<'static>>;

        unsafe fn from_cxx_string<'a>(str: &'a CxxString) -> SharedPtr<CxxTwine<'a>>;

        unsafe fn from_string_ref<'a>(str: &CxxStringRef<'a>) -> SharedPtr<CxxTwine<'a>>;
    }
}

use self::ffi::CxxTwine;
use crate::llvm::StringRef;
use cxx::{CxxString, SharedPtr};

#[derive(Clone)]
pub struct Twine<'a> {
    pub(crate) ptr: SharedPtr<CxxTwine<'a>>,
}

impl<'a> From<SharedPtr<CxxTwine<'a>>> for Twine<'a> {
    #[inline]
    fn from(ptr: SharedPtr<CxxTwine<'a>>) -> Self {
        Self { ptr }
    }
}

impl<'a> From<&'a CxxString> for Twine<'a> {
    #[inline]
    fn from(value: &'a CxxString) -> Self {
        let ptr = unsafe { self::ffi::from_cxx_string(value) };
        Self { ptr }
    }
}

impl<'a> From<&StringRef<'a>> for Twine<'a> {
    #[inline]
    fn from(value: &StringRef<'a>) -> Self {
        let ptr = unsafe { self::ffi::from_string_ref(&value.ptr) };
        Self { ptr }
    }
}

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
        let ptr = self::ffi::from_string_ref(&str.ptr);
        Self { ptr }
    }
}
