#[cxx::bridge]
pub(crate) mod ffi {
    #[derive(Clone)]
    #[namespace = "rust::llvm"]
    struct StringRef {
        ptr: SharedPtr<CxxStringRef>,
    }

    #[namespace = "llvm"]
    extern "C++" {
        include!("llvm/ADT/StringRef.h");

        #[cxx_name = "StringRef"]
        type CxxStringRef;
    }

    // String Operations
    extern "C++" {
        unsafe fn data(self: &CxxStringRef) -> *const c_char;

        unsafe fn empty(self: &CxxStringRef) -> bool;

        unsafe fn size(self: &CxxStringRef) -> usize;

        unsafe fn front(self: &CxxStringRef) -> c_char;

        unsafe fn back(self: &CxxStringRef) -> c_char;
    }

    // String Searching
    extern "C++" {
        unsafe fn find(self: &CxxStringRef, C: c_char, From: usize) -> usize;
    }

    // Helpful Algorithms
    extern "C++" {
        unsafe fn count(self: &CxxStringRef, C: c_char) -> usize;
    }

    // Fixes
    #[namespace = "cxx::llvm::StringRef"]
    extern "C++" {
        include!("cxx/llvm/StringRef.hxx");

        unsafe fn make() -> SharedPtr<CxxStringRef>;

        unsafe fn from_cxx_string(str: &CxxString) -> SharedPtr<CxxStringRef>;

        unsafe fn equals(lhs: &CxxStringRef, rhs: &CxxStringRef) -> bool;

        unsafe fn equals_insensitive(lhs: &CxxStringRef, rhs: &CxxStringRef) -> bool;
    }
}

use self::ffi::StringRef;
use core::ffi::c_char;

impl StringRef {
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
}

impl StringRef {
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

impl StringRef {
    #[inline]
    pub unsafe fn find(&self, c: c_char, from: usize) -> usize {
        self.ptr.find(c, from)
    }
}

impl StringRef {
    #[inline]
    pub unsafe fn count(&self, c: c_char) -> usize {
        self.ptr.count(c)
    }
}

impl StringRef {
    #[inline]
    pub unsafe fn equals(&self, that: &Self) -> bool {
        ffi::equals(&self.ptr, &that.ptr)
    }

    #[inline]
    pub unsafe fn equals_insensitive(&self, that: &Self) -> bool {
        ffi::equals_insensitive(&self.ptr, &that.ptr)
    }
}
