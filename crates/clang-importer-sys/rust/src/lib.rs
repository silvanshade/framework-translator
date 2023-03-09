mod ffi {
    pub(crate) mod clang;
    pub(crate) mod llvm;
    pub(crate) mod swift;
}

pub mod clang {
    pub mod tooling {}
}

pub mod llvm {
    pub use crate::ffi::llvm::{string_ref::ffi::StringRef, triple::ffi::Triple, twine::ffi::Twine};

    pub mod triple {
        pub use crate::ffi::llvm::triple::ffi::{
            ArchType,
            EnvironmentType,
            OSType,
            ObjectFormatType,
            SubArchType,
            VendorType,
        };
    }
}

pub mod swift {
    pub use crate::ffi::swift::{
        ast_context::ffi::ASTContext,
        clang_importer::ffi::ClangImporter,
        clang_importer_options::ffi::ClangImporterOptions,
        diagnostic_engine::ffi::DiagnosticEngine,
        function_body_skipping::ffi::FunctionBodySkipping,
        lang_options::ffi::LangOptions,
        search_path_options::ffi::SearchPathOptions,
        sil_options::ffi::SILOptions,
        source_manager::ffi::SourceManager,
        type_checker_options::ffi::TypeCheckerOptions,
    };

    pub mod clang_importer_options {
        pub use crate::ffi::swift::clang_importer_options::ffi::Modes;
    }

    pub mod symbolgraphgen {
        pub use crate::ffi::swift::symbolgraphgen::symbol_graph_options::ffi::SymbolGraphOptions;
    }
}
