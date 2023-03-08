use clang_importer_sys::swift::symbolgraphgen::SymbolGraphOptions;

#[test]
fn new() {
    unsafe {
        let _ = SymbolGraphOptions::new();
    }
}
