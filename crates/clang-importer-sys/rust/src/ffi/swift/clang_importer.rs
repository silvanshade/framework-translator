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

        fn create(ctx: &mut ASTContext) -> UniquePtr<CxxClangImporter>;
    }
}

use self::ffi::{ASTContext, ClangImporter};

impl ClangImporter {
    #[inline]
    pub fn create(ctx: &mut ASTContext) -> Self {
        let ptr = self::ffi::create(ctx);
        Self { ptr }
    }
}
