#![allow(clippy::too_many_arguments)]

#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "swift"]
    extern "C++" {
        include!("swift/AST/ASTContext.h");

        #[cxx_name = "ASTContext"]
        type CxxASTContext;
    }

    #[namespace = "cxx::swift::ASTContext"]
    extern "C++" {
        include!("cxx/swift/ASTContext.hxx");

        #[namespace = "swift"]
        #[cxx_name = "ClangImporterOptions"]
        type CxxClangImporterOptions = crate::ffi::swift::clang_importer_options::ffi::CxxClangImporterOptions;

        #[namespace = "swift"]
        #[cxx_name = "DiagnosticEngine"]
        type CxxDiagnosticEngine = crate::ffi::swift::diagnostic_engine::ffi::CxxDiagnosticEngine;

        #[namespace = "swift"]
        #[cxx_name = "LangOptions"]
        type CxxLangOptions = crate::ffi::swift::lang_options::ffi::CxxLangOptions;

        #[namespace = "swift"]
        #[cxx_name = "SearchPathOptions"]
        type CxxSearchPathOptions = crate::ffi::swift::search_path_options::ffi::CxxSearchPathOptions;

        #[namespace = "swift"]
        #[cxx_name = "SILOptions"]
        type CxxSILOptions = crate::ffi::swift::sil_options::ffi::CxxSILOptions;

        #[namespace = "swift"]
        #[cxx_name = "SourceManager"]
        type CxxSourceManager = crate::ffi::swift::source_manager::ffi::CxxSourceManager;

        #[namespace = "swift::symbolgraphgen"]
        #[cxx_name = "SymbolGraphOptions"]
        type CxxSymbolGraphOptions =
            crate::ffi::swift::symbolgraphgen::symbol_graph_options::ffi::CxxSymbolGraphOptions;

        #[namespace = "swift"]
        #[cxx_name = "TypeCheckerOptions"]
        type CxxTypeCheckerOptions = crate::ffi::swift::type_checker_options::ffi::CxxTypeCheckerOptions;

        unsafe fn get(
            lang_opts: Pin<&mut CxxLangOptions>,
            typeck_opts: Pin<&mut CxxTypeCheckerOptions>,
            sil_opts: Pin<&mut CxxSILOptions>,
            search_path_opts: Pin<&mut CxxSearchPathOptions>,
            clang_importer_opts: Pin<&mut CxxClangImporterOptions>,
            symbol_graph_opts: Pin<&mut CxxSymbolGraphOptions>,
            source_mgr: Pin<&mut CxxSourceManager>,
            diags: Pin<&mut CxxDiagnosticEngine>,
        ) -> UniquePtr<CxxASTContext>;

        unsafe fn getWithCallback(
            lang_opts: Pin<&mut CxxLangOptions>,
            typeck_opts: Pin<&mut CxxTypeCheckerOptions>,
            sil_opts: Pin<&mut CxxSILOptions>,
            search_path_opts: Pin<&mut CxxSearchPathOptions>,
            clang_importer_opts: Pin<&mut CxxClangImporterOptions>,
            symbol_graph_opts: Pin<&mut CxxSymbolGraphOptions>,
            source_mgr: Pin<&mut CxxSourceManager>,
            diags: Pin<&mut CxxDiagnosticEngine>,
            pre_module_import_callback: fn(&str, bool) -> bool,
        ) -> UniquePtr<CxxASTContext>;
    }
}

use self::ffi::CxxASTContext;
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
use cxx::UniquePtr;

pub struct ASTContext {
    pub(crate) ptr: UniquePtr<CxxASTContext>,
}

impl From<UniquePtr<CxxASTContext>> for ASTContext {
    #[inline]
    fn from(ptr: UniquePtr<CxxASTContext>) -> Self {
        Self { ptr }
    }
}

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
            lang_opts.ptr.pin_mut(),
            typeck_opts.ptr.pin_mut(),
            sil_opts.ptr.pin_mut(),
            search_path_opts.ptr.pin_mut(),
            clang_importer_opts.ptr.pin_mut(),
            symbol_graph_opts.ptr.pin_mut(),
            source_mgr.ptr.pin_mut(),
            diags.ptr.pin_mut(),
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
        pre_module_import_callback: fn(&str, bool) -> bool,
    ) -> ASTContext {
        let ptr = self::ffi::getWithCallback(
            lang_opts.ptr.pin_mut(),
            typeck_opts.ptr.pin_mut(),
            sil_opts.ptr.pin_mut(),
            search_path_opts.ptr.pin_mut(),
            clang_importer_opts.ptr.pin_mut(),
            symbol_graph_opts.ptr.pin_mut(),
            source_mgr.ptr.pin_mut(),
            diags.ptr.pin_mut(),
            pre_module_import_callback,
        );
        Self { ptr }
    }
}
