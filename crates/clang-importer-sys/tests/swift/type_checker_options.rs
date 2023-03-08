use clang_importer_sys::swift::TypeCheckerOptions;

#[test]
fn new() {
    unsafe {
        let _ = TypeCheckerOptions::new();
    }
}
