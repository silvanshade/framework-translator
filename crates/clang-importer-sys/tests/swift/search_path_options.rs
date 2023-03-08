use clang_importer_sys::swift::SearchPathOptions;

#[test]
fn new() {
    unsafe {
        let _ = SearchPathOptions::new();
    }
}
