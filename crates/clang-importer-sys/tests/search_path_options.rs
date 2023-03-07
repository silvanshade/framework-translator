mod common;

use clang_importer_sys::swift::SearchPathOptions;

#[test]
fn new() {
    let _ = SearchPathOptions::new();
}
