#[cxx::bridge]
pub(crate) mod ffi {
    #[derive(Clone)]
    #[namespace = "rust::llvm"]
    struct StringRef<'a> {
        ptr: SharedPtr<CxxStringRef<'a>>,
    }

    #[namespace = "llvm"]
    extern "C++" {
        include!("llvm/ADT/StringRef.h");

        #[cxx_name = "StringRef"]
        type CxxStringRef<'a>;
    }

    // String Operations
    extern "C++" {
        unsafe fn data(self: &CxxStringRef<'_>) -> *const c_char;

        unsafe fn empty(self: &CxxStringRef<'_>) -> bool;

        unsafe fn size(self: &CxxStringRef<'_>) -> usize;

        unsafe fn front(self: &CxxStringRef<'_>) -> c_char;

        unsafe fn back(self: &CxxStringRef<'_>) -> c_char;
    }

    // String Searching
    extern "C++" {
        unsafe fn find(self: &CxxStringRef<'_>, C: c_char, From: usize) -> usize;
    }

    // Helpful Algorithms
    extern "C++" {
        unsafe fn count(self: &CxxStringRef<'_>, C: c_char) -> usize;
    }

    // Fixes
    #[namespace = "cxx::llvm::StringRef"]
    unsafe extern "C++" {
        include!("cxx/llvm/StringRef.hxx");

        unsafe fn make() -> SharedPtr<CxxStringRef<'static>>;

        unsafe fn from_cxx_string<'a>(str: &'a CxxString) -> SharedPtr<CxxStringRef<'a>>;

        unsafe fn equals(lhs: &CxxStringRef<'_>, rhs: &CxxStringRef<'_>) -> bool;

        unsafe fn equals_insensitive(lhs: &CxxStringRef<'_>, rhs: &CxxStringRef<'_>) -> bool;
    }
}

use self::ffi::{CxxStringRef, StringRef};
use core::ffi::c_char;
use cxx::{CxxString, SharedPtr};

impl<'a> From<SharedPtr<CxxStringRef<'a>>> for StringRef<'a> {
    #[inline]
    fn from(ptr: SharedPtr<CxxStringRef<'a>>) -> Self {
        Self { ptr }
    }
}

impl<'a> StringRef<'a> {
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
}

impl<'a> StringRef<'a> {
    #[inline]
    pub unsafe fn data(&self) -> *const c_char {
        self.ptr.data()
    }

    #[inline]
    pub unsafe fn empty(&self) -> bool {
        self.ptr.empty()
    }

    #[inline]
    pub unsafe fn size(&self) -> usize {
        self.ptr.size()
    }

    #[inline]
    pub unsafe fn front(&self) -> c_char {
        self.ptr.front()
    }

    #[inline]
    pub unsafe fn back(&self) -> c_char {
        self.ptr.back()
    }
}

impl<'a> StringRef<'a> {
    #[inline]
    pub unsafe fn find(&self, c: c_char, from: usize) -> usize {
        self.ptr.find(c, from)
    }
}

impl<'a> StringRef<'a> {
    #[inline]
    pub unsafe fn count(&self, c: c_char) -> usize {
        self.ptr.count(c)
    }
}

impl<'a> StringRef<'a> {
    #[inline]
    pub unsafe fn equals(&self, that: &Self) -> bool {
        ffi::equals(&self.ptr, &that.ptr)
    }

    #[inline]
    pub unsafe fn equals_insensitive(&self, that: &Self) -> bool {
        ffi::equals_insensitive(&self.ptr, &that.ptr)
    }
}
