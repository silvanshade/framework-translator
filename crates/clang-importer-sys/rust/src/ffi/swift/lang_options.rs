#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "swift"]
    extern "C++" {
        include!("swift/Basic/LangOptions.h");

        #[cxx_name = "LangOptions"]
        type CxxLangOptions;
    }

    #[namespace = "cxx::swift::LangOptions"]
    extern "C++" {
        include!("cxx/swift/LangOptions.hxx");

        unsafe fn make() -> UniquePtr<CxxLangOptions>;

        #[namespace = "llvm"]
        #[cxx_name = "Triple"]
        type CxxTriple = crate::ffi::llvm::triple::ffi::CxxTriple;

        unsafe fn Target(This: &CxxLangOptions) -> UniquePtr<CxxTriple>;

        unsafe fn SetTarget(This: Pin<&mut CxxLangOptions>, target: UniquePtr<CxxTriple>);
    }
}

use self::ffi::CxxLangOptions;
use crate::llvm::Triple;
use cxx::UniquePtr;

pub struct LangOptions {
    pub(crate) ptr: UniquePtr<CxxLangOptions>,
}

impl LangOptions {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }

    #[inline]
    pub unsafe fn target(&self) -> Triple {
        let this = &self.ptr;
        let ptr = self::ffi::Target(this);
        Triple { ptr }
    }

    #[inline]
    pub unsafe fn set_target(&mut self, target: Triple) {
        let this = self.ptr.pin_mut();
        self::ffi::SetTarget(this, target.ptr);
    }
}
