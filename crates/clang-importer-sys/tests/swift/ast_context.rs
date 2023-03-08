use clang_importer_sys::swift::{
    symbolgraphgen::SymbolGraphOptions,
    ASTContext,
    ClangImporterOptions,
    DiagnosticEngine,
    LangOptions,
    SILOptions,
    SearchPathOptions,
    SourceManager,
    TypeCheckerOptions,
};

#[test]
fn new() {
    #![allow(non_snake_case)]
    let mut lang_opts = LangOptions::new();
    let mut typeck_opts = TypeCheckerOptions::new();
    let mut sil_opts = SILOptions::new();
    let mut search_path_opts = SearchPathOptions::new();
    let mut clang_importer_opts = ClangImporterOptions::new();
    let mut symbol_graph_opts = SymbolGraphOptions::new();
    let mut source_mgr = SourceManager::new();
    let mut diags = DiagnosticEngine::from(&mut source_mgr);
    let _ = ASTContext::get(
        &mut lang_opts,
        &mut typeck_opts,
        &mut sil_opts,
        &mut search_path_opts,
        &mut clang_importer_opts,
        &mut symbol_graph_opts,
        &mut source_mgr,
        &mut diags,
    );
}
