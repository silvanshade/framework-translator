use clang_importer_sys::swift::ClangImporterOptions;

#[test]
fn new() {
    unsafe {
        let _ = ClangImporterOptions::new();
    }
}
