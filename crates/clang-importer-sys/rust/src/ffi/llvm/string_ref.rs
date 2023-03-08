#[cxx::bridge]
pub(crate) mod ffi {
    #[derive(Clone)]
    #[namespace = "rust::llvm"]
    struct StringRef {
        ptr: SharedPtr<CxxStringRef>,
    }

    #[namespace = "llvm"]
    unsafe extern "C++" {
        include!("llvm/ADT/StringRef.h");

        #[cxx_name = "StringRef"]
        type CxxStringRef;
    }

    // String Operations
    unsafe extern "C++" {
        fn data(self: &CxxStringRef) -> *const c_char;

        fn empty(self: &CxxStringRef) -> bool;

        fn size(self: &CxxStringRef) -> usize;

        fn front(self: &CxxStringRef) -> c_char;

        fn back(self: &CxxStringRef) -> c_char;
    }

    // String Searching
    unsafe extern "C++" {
        fn find(self: &CxxStringRef, C: c_char, From: usize) -> usize;
    }

    // Helpful Algorithms
    unsafe extern "C++" {
        fn count(self: &CxxStringRef, C: c_char) -> usize;
    }

    // Fixes
    #[namespace = "cxx::llvm::StringRef"]
    unsafe extern "C++" {
        include!("cxx/llvm/StringRef.hxx");

        fn make() -> SharedPtr<CxxStringRef>;

        fn from_cxx_string(Str: &CxxString) -> SharedPtr<CxxStringRef>;

        fn equals(LHS: &CxxStringRef, RHS: &CxxStringRef) -> bool;

        fn equals_insensitive(LHS: &CxxStringRef, RHS: &CxxStringRef) -> bool;
    }
}

use self::ffi::StringRef;
use core::ffi::c_char;
use cxx::CxxString;

impl StringRef {
    #[inline]
    pub fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }
}

impl Default for StringRef {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<&CxxString> for StringRef {
    #[inline]
    fn from(value: &cxx::CxxString) -> Self {
        let ptr = self::ffi::from_cxx_string(value);
        Self { ptr }
    }
}

impl StringRef {
    #[inline]
    pub fn data(&self) -> *const c_char {
        self.ptr.data()
    }

    #[inline]
    pub fn empty(&self) -> bool {
        self.ptr.empty()
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.ptr.size()
    }

    #[inline]
    pub fn front(&self) -> c_char {
        self.ptr.front()
    }

    #[inline]
    pub fn back(&self) -> c_char {
        self.ptr.back()
    }
}

impl StringRef {
    #[inline]
    #[allow(non_snake_case)]
    pub fn find(&self, C: c_char, From: usize) -> usize {
        self.ptr.find(C, From)
    }
}

impl StringRef {
    #[inline]
    #[allow(non_snake_case)]
    pub fn count(&self, C: c_char) -> usize {
        self.ptr.count(C)
    }
}

impl StringRef {
    #[inline]
    #[allow(non_snake_case)]
    pub fn equals(&self, RHS: &Self) -> bool {
        ffi::equals(&self.ptr, &RHS.ptr)
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn equals_insensitive(&self, RHS: &Self) -> bool {
        ffi::equals_insensitive(&self.ptr, &RHS.ptr)
    }
}
