#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct ClangImporter {
        ptr: UniquePtr<CxxClangImporter>,
    }

    #[namespace = "swift"]
    extern "C++" {
        include!("swift/ClangImporter/ClangImporter.h");

        #[cxx_name = "ClangImporter"]
        type CxxClangImporter;
    }

    #[namespace = "cxx::swift::ClangImporter"]
    extern "C++" {
        include!("cxx/swift/ClangImporter.hxx");

        #[namespace = "rust::swift"]
        type ASTContext = crate::swift::ASTContext;

        #[namespace = "rust::llvm"]
        type StringRef<'a> = crate::llvm::StringRef<'a>;

        unsafe fn create(ctx: &mut ASTContext) -> UniquePtr<CxxClangImporter>;

        unsafe fn canReadPCH(This: Pin<&mut CxxClangImporter>, pch_filename: &StringRef<'_>) -> bool;

        unsafe fn emitBridgingPCH(
            This: Pin<&mut CxxClangImporter>,
            header_path: &StringRef<'_>,
            output_pch_path: &StringRef<'_>,
        ) -> bool;
    }
}

use self::ffi::{ASTContext, ClangImporter, CxxClangImporter};
use crate::llvm::StringRef;
use cxx::UniquePtr;

impl From<UniquePtr<CxxClangImporter>> for ClangImporter {
    #[inline]
    fn from(ptr: UniquePtr<CxxClangImporter>) -> Self {
        Self { ptr }
    }
}

impl ClangImporter {
    #[inline]
    pub unsafe fn create(ctx: &mut ASTContext) -> Self {
        let ptr = self::ffi::create(ctx);
        Self { ptr }
    }

    #[inline]
    pub unsafe fn can_read_pch(&mut self, pch_filename: &StringRef<'_>) -> bool {
        let this = self.ptr.pin_mut();
        self::ffi::canReadPCH(this, pch_filename)
    }

    #[inline]
    pub unsafe fn emit_bridging_pch(&mut self, header_path: &StringRef<'_>, output_pch_path: &StringRef<'_>) -> bool {
        let this = self.ptr.pin_mut();
        self::ffi::emitBridgingPCH(this, header_path, output_pch_path)
    }
}
