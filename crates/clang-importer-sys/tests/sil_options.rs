mod common;

use clang_importer_sys::swift::SILOptions;

#[test]
fn new() {
    let _ = SILOptions::new();
}
