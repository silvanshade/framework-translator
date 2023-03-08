use clang_importer_sys::swift::SILOptions;

#[test]
fn new() {
    unsafe {
        let _ = SILOptions::new();
    }
}
