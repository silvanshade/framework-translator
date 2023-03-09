#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct DiagnosticEngine {
        ptr: UniquePtr<CxxDiagnosticEngine>,
    }

    #[namespace = "swift"]
    extern "C++" {
        include!("swift/AST/DiagnosticEngine.h");

        #[cxx_name = "DiagnosticEngine"]
        type CxxDiagnosticEngine;
    }

    #[namespace = "cxx::swift::DiagnosticEngine"]
    extern "C++" {
        include!("cxx/swift/DiagnosticEngine.hxx");

        #[namespace = "rust::swift"]
        type SourceManager = crate::swift::SourceManager;

        unsafe fn make(source_mgr: &mut SourceManager) -> UniquePtr<CxxDiagnosticEngine>;
    }
}

use self::ffi::{CxxDiagnosticEngine, DiagnosticEngine};
use crate::swift::SourceManager;
use cxx::UniquePtr;

impl From<UniquePtr<CxxDiagnosticEngine>> for DiagnosticEngine {
    #[inline]
    fn from(ptr: UniquePtr<CxxDiagnosticEngine>) -> Self {
        Self { ptr }
    }
}

impl DiagnosticEngine {
    pub unsafe fn new(source_mgr: &mut SourceManager) -> Self {
        let ptr = self::ffi::make(source_mgr);
        Self { ptr }
    }
}
