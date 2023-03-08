use clang_importer_sys::{
    llvm::{Triple, Twine},
    swift::{
        symbolgraphgen::SymbolGraphOptions,
        ASTContext,
        ClangImporter,
        ClangImporterOptions,
        DiagnosticEngine,
        LangOptions,
        SILOptions,
        SearchPathOptions,
        SourceManager,
        TypeCheckerOptions,
    },
};

#[test]
fn create() {
    #![allow(non_snake_case)]
    let mut langOpts = LangOptions::new();
    cxx::let_cxx_string!(arch = "x86_64");
    cxx::let_cxx_string!(vendor = "apple");
    cxx::let_cxx_string!(os = "darwin");
    let arch = Twine::from(&*arch);
    let vendor = Twine::from(&*vendor);
    let os = Twine::from(&*os);
    langOpts.set_target(Triple::from_arch_vendor_os(&arch, &vendor, &os));
    let mut typeckOpts = TypeCheckerOptions::new();
    let mut silOpts = SILOptions::new();
    let mut SearchPathOpts = SearchPathOptions::new();
    let mut ClangImporterOpts = ClangImporterOptions::new();
    let mut SymbolGraphOpts = SymbolGraphOptions::new();
    let mut SourceMgr = SourceManager::new();
    let mut Diags = DiagnosticEngine::from(&mut SourceMgr);
    let mut ctx = ASTContext::get(
        &mut langOpts,
        &mut typeckOpts,
        &mut silOpts,
        &mut SearchPathOpts,
        &mut ClangImporterOpts,
        &mut SymbolGraphOpts,
        &mut SourceMgr,
        &mut Diags,
    );
    let _ = ClangImporter::create(&mut ctx);
}
