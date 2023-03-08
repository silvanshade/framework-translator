use clang_importer_sys::llvm::Triple;

#[test]
fn new() {
    unsafe {
        let _ = Triple::new();
    }
}
