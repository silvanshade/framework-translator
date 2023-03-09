#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "swift"]
    extern "C++" {
        include!("swift/AST/DiagnosticEngine.h");

        #[cxx_name = "DiagnosticEngine"]
        type CxxDiagnosticEngine;
    }

    #[namespace = "cxx::swift::DiagnosticEngine"]
    extern "C++" {
        include!("cxx/swift/DiagnosticEngine.hxx");

        #[namespace = "swift"]
        #[cxx_name = "SourceManager"]
        type CxxSourceManager = crate::ffi::swift::source_manager::ffi::CxxSourceManager;

        unsafe fn make(source_mgr: Pin<&mut CxxSourceManager>) -> UniquePtr<CxxDiagnosticEngine>;
    }
}

use self::ffi::CxxDiagnosticEngine;
use crate::swift::SourceManager;
use cxx::UniquePtr;

pub struct DiagnosticEngine {
    pub(crate) ptr: UniquePtr<CxxDiagnosticEngine>,
}

impl From<UniquePtr<CxxDiagnosticEngine>> for DiagnosticEngine {
    #[inline]
    fn from(ptr: UniquePtr<CxxDiagnosticEngine>) -> Self {
        Self { ptr }
    }
}

impl DiagnosticEngine {
    pub unsafe fn new(source_mgr: &mut SourceManager) -> Self {
        let ptr = self::ffi::make(source_mgr.ptr.pin_mut());
        Self { ptr }
    }
}
