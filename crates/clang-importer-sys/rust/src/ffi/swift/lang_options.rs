#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct LangOptions {
        ptr: UniquePtr<CxxLangOptions>,
    }

    #[namespace = "swift"]
    unsafe extern "C++" {
        include!("swift/Basic/LangOptions.h");

        #[cxx_name = "LangOptions"]
        type CxxLangOptions;
    }

    #[namespace = "cxx::swift::LangOptions"]
    unsafe extern "C++" {
        include!("cxx/swift/LangOptions.hxx");

        fn make() -> UniquePtr<CxxLangOptions>;

        #[namespace = "llvm"]
        #[cxx_name = "Triple"]
        type CxxTriple = crate::ffi::llvm::triple::ffi::CxxTriple;

        fn Target(This: &CxxLangOptions) -> UniquePtr<CxxTriple>;

        fn SetTarget(This: Pin<&mut CxxLangOptions>, target: UniquePtr<CxxTriple>);
    }
}

use self::ffi::LangOptions;
use crate::llvm::Triple;

impl LangOptions {
    #[inline]
    pub fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }

    #[inline]
    pub fn target(&self) -> Triple {
        let this = &self.ptr;
        let ptr = self::ffi::Target(this);
        Triple { ptr }
    }

    #[inline]
    pub fn set_target(&mut self, target: Triple) {
        let this = self.ptr.pin_mut();
        self::ffi::SetTarget(this, target.ptr);
    }
}
