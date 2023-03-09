#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "llvm"]
    extern "C++" {
        include!("llvm/ADT/Hashing.h");

        #[cxx_name = "hash_code"]
        type CxxHashCode;
    }

    #[namespace = "cxx::llvm::HashCode"]
    extern "C++" {
        include!("cxx/llvm/HashCode.hxx");

        unsafe fn make() -> UniquePtr<CxxHashCode>;

        unsafe fn value(This: &CxxHashCode) -> usize;
    }
}

use self::ffi::CxxHashCode;
use cxx::UniquePtr;

pub struct HashCode {
    pub(crate) ptr: UniquePtr<CxxHashCode>,
}

impl From<HashCode> for usize {
    fn from(value: HashCode) -> Self {
        unsafe { self::ffi::value(&value.ptr) }
    }
}

impl HashCode {
    pub fn new() -> Self {
        let ptr = unsafe { self::ffi::make() };
        Self { ptr }
    }
}
