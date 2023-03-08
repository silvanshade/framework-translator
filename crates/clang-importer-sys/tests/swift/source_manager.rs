use clang_importer_sys::swift::SourceManager;

#[test]
fn new() {
    unsafe {
        let _ = SourceManager::new();
    }
}
