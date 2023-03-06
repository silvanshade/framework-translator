#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct ASTContext {
        ptr: UniquePtr<CxxASTContext>,
    }

    #[namespace = "swift"]
    unsafe extern "C++" {
        include!("swift/AST/ASTContext.h");

        #[cxx_name = "ASTContext"]
        type CxxASTContext;
    }

    #[namespace = "cxx::swift::ASTContext"]
    unsafe extern "C++" {
        include!("cxx/swift/ASTContext.hxx");

        #[namespace = "rust::swift"]
        type ClangImporterOptions = crate::swift::ClangImporterOptions;

        #[namespace = "rust::swift"]
        type DiagnosticEngine = crate::swift::DiagnosticEngine;

        #[namespace = "rust::swift"]
        type LangOptions = crate::swift::LangOptions;

        #[namespace = "rust::swift"]
        type SearchPathOptions = crate::swift::SearchPathOptions;

        #[namespace = "rust::swift"]
        type SILOptions = crate::swift::SILOptions;

        #[namespace = "rust::swift"]
        type SourceManager = crate::swift::SourceManager;

        #[namespace = "rust::llvm"]
        type StringRef = crate::llvm::StringRef;

        #[namespace = "rust::swift::symbolgraphgen"]
        type SymbolGraphOptions = crate::swift::symbolgraphgen::SymbolGraphOptions;

        #[namespace = "rust::swift"]
        type TypeCheckerOptions = crate::swift::TypeCheckerOptions;

        fn get(
            langOpts: &mut LangOptions,
            typeckOpts: &mut TypeCheckerOptions,
            silOpts: &mut SILOptions,
            SearchPathOpts: &mut SearchPathOptions,
            ClangImporterOpts: &mut ClangImporterOptions,
            SymbolGraphOpts: &mut SymbolGraphOptions,
            SourceMgr: &mut SourceManager,
            Diags: &mut DiagnosticEngine,
        ) -> UniquePtr<CxxASTContext>;

        #[allow(unused)]
        fn getWithCallback(
            langOpts: &mut LangOptions,
            typeckOpts: &mut TypeCheckerOptions,
            silOpts: &mut SILOptions,
            SearchPathOpts: &mut SearchPathOptions,
            ClangImporterOpts: &mut ClangImporterOptions,
            SymbolGraphOpts: &mut SymbolGraphOptions,
            SourceMgr: &mut SourceManager,
            Diags: &mut DiagnosticEngine,
            PreModuleImportCallback: fn(StringRef, bool) -> bool,
        ) -> UniquePtr<CxxASTContext>;
    }
}

use self::ffi::ASTContext;
use crate::swift::{
    symbolgraphgen::SymbolGraphOptions,
    ClangImporterOptions,
    DiagnosticEngine,
    LangOptions,
    SILOptions,
    SearchPathOptions,
    SourceManager,
    TypeCheckerOptions,
};

impl ASTContext {
    #[inline]
    #[allow(non_snake_case)]
    pub fn get(
        langOpts: &mut LangOptions,
        typeckOpts: &mut TypeCheckerOptions,
        silOpts: &mut SILOptions,
        SearchPathOpts: &mut SearchPathOptions,
        ClangImporterOpts: &mut ClangImporterOptions,
        SymbolGraphOpts: &mut SymbolGraphOptions,
        SourceMgr: &mut SourceManager,
        Diags: &mut DiagnosticEngine,
    ) -> ASTContext {
        let ptr = self::ffi::get(
            langOpts,
            typeckOpts,
            silOpts,
            SearchPathOpts,
            ClangImporterOpts,
            SymbolGraphOpts,
            SourceMgr,
            Diags,
        );
        Self { ptr }
    }
}
