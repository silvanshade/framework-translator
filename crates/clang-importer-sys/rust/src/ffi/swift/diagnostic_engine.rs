#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct DiagnosticEngine {
        ptr: UniquePtr<CxxDiagnosticEngine>,
    }

    #[namespace = "swift"]
    unsafe extern "C++" {
        include!("swift/AST/DiagnosticEngine.h");

        #[cxx_name = "DiagnosticEngine"]
        type CxxDiagnosticEngine;
    }

    #[namespace = "cxx::swift::DiagnosticEngine"]
    unsafe extern "C++" {
        include!("cxx/swift/DiagnosticEngine.hxx");

        #[namespace = "rust::swift"]
        type SourceManager = crate::swift::SourceManager;

        fn make(SourceMgr: &mut SourceManager) -> UniquePtr<CxxDiagnosticEngine>;
    }
}

use self::ffi::DiagnosticEngine;
use crate::swift::SourceManager;

impl From<&mut SourceManager> for DiagnosticEngine {
    #[inline]
    fn from(value: &mut SourceManager) -> Self {
        let ptr = self::ffi::make(value);
        Self { ptr }
    }
}
