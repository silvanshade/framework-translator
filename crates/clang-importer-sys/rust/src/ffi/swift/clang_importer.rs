#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct ClangImporter {
        ptr: UniquePtr<CxxClangImporter>,
    }

    #[namespace = "swift"]
    unsafe extern "C++" {
        include!("swift/ClangImporter/ClangImporter.h");

        #[cxx_name = "ClangImporter"]
        type CxxClangImporter;
    }

    #[namespace = "cxx::swift::ClangImporter"]
    unsafe extern "C++" {
        include!("cxx/swift/ClangImporter.hxx");

        #[namespace = "rust::swift"]
        type ASTContext = crate::swift::ASTContext;

        #[namespace = "rust::llvm"]
        type StringRef = crate::llvm::StringRef;

        fn create(ctx: &mut ASTContext) -> UniquePtr<CxxClangImporter>;

        fn canReadPCH(This: Pin<&mut CxxClangImporter>, pch_filename: &StringRef) -> bool;

        fn emitBridgingPCH(
            This: Pin<&mut CxxClangImporter>,
            header_path: &StringRef,
            output_pch_path: &StringRef,
        ) -> bool;
    }
}

use self::ffi::{ASTContext, ClangImporter};
use crate::llvm::StringRef;

impl ClangImporter {
    #[inline]
    pub fn create(ctx: &mut ASTContext) -> Self {
        let ptr = self::ffi::create(ctx);
        Self { ptr }
    }

    #[inline]
    pub fn can_read_pch(&mut self, pch_filename: &StringRef) -> bool {
        let this = self.ptr.pin_mut();
        self::ffi::canReadPCH(this, pch_filename)
    }

    #[inline]
    pub fn emit_bridging_pch(&mut self, header_path: &StringRef, output_pch_path: &StringRef) -> bool {
        let this = self.ptr.pin_mut();
        self::ffi::emitBridgingPCH(this, header_path, output_pch_path)
    }
}
