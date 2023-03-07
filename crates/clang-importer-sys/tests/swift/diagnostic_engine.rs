use clang_importer_sys::swift::{DiagnosticEngine, SourceManager};

#[test]
fn new() {
    let mut source_manager = SourceManager::new();
    let _ = DiagnosticEngine::from(&mut source_manager);
}
