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

use self::ffi::DiagnosticEngine;
use crate::swift::SourceManager;

impl DiagnosticEngine {
    pub unsafe fn new(source_mgr: &mut SourceManager) -> Self {
        let ptr = self::ffi::make(source_mgr);
        Self { ptr }
    }
}
