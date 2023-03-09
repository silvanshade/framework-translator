mod ffi {
    pub(crate) mod clang;
    pub(crate) mod llvm;
    pub(crate) mod swift;
}

pub mod clang {
    pub mod tooling {}
}

pub mod llvm {
    pub use crate::ffi::llvm::{string_ref::StringRef, triple::Triple, twine::Twine};

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
        ast_context::ASTContext,
        clang_importer::ClangImporter,
        clang_importer_options::ClangImporterOptions,
        diagnostic_engine::DiagnosticEngine,
        function_body_skipping::ffi::FunctionBodySkipping,
        lang_options::LangOptions,
        search_path_options::SearchPathOptions,
        sil_options::SILOptions,
        source_manager::SourceManager,
        type_checker_options::TypeCheckerOptions,
    };

    pub mod clang_importer_options {
        pub use crate::ffi::swift::clang_importer_options::ffi::Modes;
    }

    pub mod symbolgraphgen {
        pub use crate::ffi::swift::symbolgraphgen::symbol_graph_options::SymbolGraphOptions;
    }
}
