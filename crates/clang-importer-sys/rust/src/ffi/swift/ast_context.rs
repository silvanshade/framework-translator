#![allow(clippy::too_many_arguments)]

#[cxx::bridge(namespace = "rust::swift")]
pub(crate) mod ffi {
    struct ASTContext {
        ptr: UniquePtr<CxxASTContext>,
    }

    #[namespace = "swift"]
    extern "C++" {
        include!("swift/AST/ASTContext.h");

        #[cxx_name = "ASTContext"]
        type CxxASTContext;
    }

    #[namespace = "cxx::swift::ASTContext"]
    extern "C++" {
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

        unsafe fn get(
            lang_opts: &mut LangOptions,
            typeck_opts: &mut TypeCheckerOptions,
            sil_opts: &mut SILOptions,
            search_path_opts: &mut SearchPathOptions,
            clang_importer_opts: &mut ClangImporterOptions,
            symbol_graph_opts: &mut SymbolGraphOptions,
            source_mgr: &mut SourceManager,
            diags: &mut DiagnosticEngine,
        ) -> UniquePtr<CxxASTContext>;

        unsafe fn getWithCallback(
            lang_opts: &mut LangOptions,
            typeck_opts: &mut TypeCheckerOptions,
            sil_opts: &mut SILOptions,
            search_path_opts: &mut SearchPathOptions,
            clang_importer_opts: &mut ClangImporterOptions,
            symbol_graph_opts: &mut SymbolGraphOptions,
            source_mgr: &mut SourceManager,
            diags: &mut DiagnosticEngine,
            pre_module_import_callback: fn(StringRef, bool) -> bool,
        ) -> UniquePtr<CxxASTContext>;
    }
}

use self::ffi::ASTContext;
use crate::{
    llvm::StringRef,
    swift::{
        symbolgraphgen::SymbolGraphOptions,
        ClangImporterOptions,
        DiagnosticEngine,
        LangOptions,
        SILOptions,
        SearchPathOptions,
        SourceManager,
        TypeCheckerOptions,
    },
};

impl ASTContext {
    #[inline]
    pub unsafe fn get(
        lang_opts: &mut LangOptions,
        typeck_opts: &mut TypeCheckerOptions,
        sil_opts: &mut SILOptions,
        search_path_opts: &mut SearchPathOptions,
        clang_importer_opts: &mut ClangImporterOptions,
        symbol_graph_opts: &mut SymbolGraphOptions,
        source_mgr: &mut SourceManager,
        diags: &mut DiagnosticEngine,
    ) -> ASTContext {
        let ptr = self::ffi::get(
            lang_opts,
            typeck_opts,
            sil_opts,
            search_path_opts,
            clang_importer_opts,
            symbol_graph_opts,
            source_mgr,
            diags,
        );
        Self { ptr }
    }

    #[inline]
    pub unsafe fn get_with_callback(
        lang_opts: &mut LangOptions,
        typeck_opts: &mut TypeCheckerOptions,
        sil_opts: &mut SILOptions,
        search_path_opts: &mut SearchPathOptions,
        clang_importer_opts: &mut ClangImporterOptions,
        symbol_graph_opts: &mut SymbolGraphOptions,
        source_mgr: &mut SourceManager,
        diags: &mut DiagnosticEngine,
        pre_module_import_callback: fn(StringRef, bool) -> bool,
    ) -> ASTContext {
        let ptr = self::ffi::getWithCallback(
            lang_opts,
            typeck_opts,
            sil_opts,
            search_path_opts,
            clang_importer_opts,
            symbol_graph_opts,
            source_mgr,
            diags,
            pre_module_import_callback,
        );
        Self { ptr }
    }
}
