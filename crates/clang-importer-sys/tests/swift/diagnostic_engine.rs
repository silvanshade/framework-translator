use clang_importer_sys::swift::{DiagnosticEngine, SourceManager};

#[test]
fn new() {
    unsafe {
        let mut source_manager = SourceManager::new();
        let _ = DiagnosticEngine::new(&mut source_manager);
    }
}
