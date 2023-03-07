use clang_importer_sys::llvm::{StringRef, Twine};
use cxx::let_cxx_string;
use tap::prelude::*;

#[test]
fn new() {
    let _ = Twine::new();
}

#[test]
fn from_cxx_string() {
    let input = "string";
    let_cxx_string!(str = input);
    let _ = Twine::from(&*str);
}

#[test]
fn from_string_ref() {
    let input = "string";
    let_cxx_string!(str = input);
    let string_ref = StringRef::from(&*str);
    let _ = Twine::from(&string_ref);
}
