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
    let mut langOpts = LangOptions::new();
    let mut typeckOpts = TypeCheckerOptions::new();
    let mut silOpts = SILOptions::new();
    let mut SearchPathOpts = SearchPathOptions::new();
    let mut ClangImporterOpts = ClangImporterOptions::new();
    let mut SymbolGraphOpts = SymbolGraphOptions::new();
    let mut SourceMgr = SourceManager::new();
    let mut Diags = DiagnosticEngine::from(&mut SourceMgr);
    let _ = ASTContext::get(
        &mut langOpts,
        &mut typeckOpts,
        &mut silOpts,
        &mut SearchPathOpts,
        &mut ClangImporterOpts,
        &mut SymbolGraphOpts,
        &mut SourceMgr,
        &mut Diags,
    );
}
