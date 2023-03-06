mod common;

use clang_importer_sys::swift::ClangImporterOptions;

#[test]
fn new() {
    let _ = ClangImporterOptions::new();
}
