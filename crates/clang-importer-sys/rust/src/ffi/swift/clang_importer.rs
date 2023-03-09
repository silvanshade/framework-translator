#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "swift"]
    extern "C++" {
        include!("swift/ClangImporter/ClangImporter.h");

        #[cxx_name = "ClangImporter"]
        type CxxClangImporter;
    }

    #[namespace = "cxx::swift::ClangImporter"]
    extern "C++" {
        include!("cxx/swift/ClangImporter.hxx");

        #[namespace = "swift"]
        #[cxx_name = "ASTContext"]
        type CxxASTContext = crate::ffi::swift::ast_context::ffi::CxxASTContext;

        #[namespace = "llvm"]
        #[cxx_name = "StringRef"]
        type CxxStringRef<'a> = crate::ffi::llvm::string_ref::ffi::CxxStringRef<'a>;

        unsafe fn create(ctx: Pin<&mut CxxASTContext>) -> UniquePtr<CxxClangImporter>;

        unsafe fn canReadPCH(This: Pin<&mut CxxClangImporter>, pch_filename: &CxxStringRef<'_>) -> bool;

        unsafe fn emitBridgingPCH(
            This: Pin<&mut CxxClangImporter>,
            header_path: &CxxStringRef<'_>,
            output_pch_path: &CxxStringRef<'_>,
        ) -> bool;
    }
}

use self::ffi::CxxClangImporter;
use crate::{llvm::StringRef, swift::ASTContext};
use cxx::UniquePtr;

pub struct ClangImporter {
    pub(crate) ptr: UniquePtr<CxxClangImporter>,
}

impl From<UniquePtr<CxxClangImporter>> for ClangImporter {
    #[inline]
    fn from(ptr: UniquePtr<CxxClangImporter>) -> Self {
        Self { ptr }
    }
}

impl ClangImporter {
    #[inline]
    pub unsafe fn create(ctx: &mut ASTContext) -> Self {
        let ptr = self::ffi::create(ctx.ptr.pin_mut());
        Self { ptr }
    }

    #[inline]
    pub unsafe fn can_read_pch(&mut self, pch_filename: &StringRef<'_>) -> bool {
        let this = self.ptr.pin_mut();
        self::ffi::canReadPCH(this, &pch_filename.ptr)
    }

    #[inline]
    pub unsafe fn emit_bridging_pch(&mut self, header_path: &StringRef<'_>, output_pch_path: &StringRef<'_>) -> bool {
        let this = self.ptr.pin_mut();
        self::ffi::emitBridgingPCH(this, &header_path.ptr, &output_pch_path.ptr)
    }
}
