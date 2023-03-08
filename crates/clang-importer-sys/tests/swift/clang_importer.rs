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
    unsafe {
        let mut lang_opts = LangOptions::new();
        cxx::let_cxx_string!(arch = "x86_64");
        cxx::let_cxx_string!(vendor = "apple");
        cxx::let_cxx_string!(os = "darwin");
        let arch = Twine::from_cxx_string(&*arch);
        let vendor = Twine::from_cxx_string(&*vendor);
        let os = Twine::from_cxx_string(&*os);
        lang_opts.set_target(Triple::from_arch_vendor_os(&arch, &vendor, &os));
        let mut typeck_opts = TypeCheckerOptions::new();
        let mut sil_opts = SILOptions::new();
        let mut search_path_opts = SearchPathOptions::new();
        let mut clang_importer_opts = ClangImporterOptions::new();
        let mut symbol_graph_opts = SymbolGraphOptions::new();
        let mut source_mgr = SourceManager::new();
        let mut diags = DiagnosticEngine::new(&mut source_mgr);
        let mut ctx = ASTContext::get(
            &mut lang_opts,
            &mut typeck_opts,
            &mut sil_opts,
            &mut search_path_opts,
            &mut clang_importer_opts,
            &mut symbol_graph_opts,
            &mut source_mgr,
            &mut diags,
        );
        let _ = ClangImporter::create(&mut ctx);
    }
}
