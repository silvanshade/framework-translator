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

        unsafe fn from_rust_str<'a>(str: &'a str) -> SharedPtr<CxxTwine<'a>>;

        unsafe fn str(This: &CxxTwine<'_>) -> UniquePtr<CxxString>;
    }
}

use self::ffi::CxxTwine;
use crate::llvm::StringRef;
use cxx::{CxxString, SharedPtr, UniquePtr};

#[derive(Clone)]
pub struct Twine<'a> {
    pub(crate) ptr: SharedPtr<CxxTwine<'a>>,
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

impl<'a> From<StringRef<'a>> for Twine<'a> {
    #[inline]
    fn from(value: StringRef<'a>) -> Self {
        let ptr = unsafe { self::ffi::from_string_ref(&value.ptr) };
        Self { ptr }
    }
}

impl<'a> From<&'a str> for Twine<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        let ptr = unsafe { self::ffi::from_rust_str(value) };
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
    pub unsafe fn str(&self) -> UniquePtr<CxxString> {
        let this = &self.ptr;
        self::ffi::str(this)
    }
}
