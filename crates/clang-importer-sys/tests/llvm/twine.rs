use clang_importer_sys::llvm::{StringRef, Twine};
use cxx::let_cxx_string;
use tap::prelude::*;

#[test]
fn new() {
    unsafe {
        let _ = Twine::new();
    }
}

#[test]
fn from_cxx_string() {
    unsafe {
        let input = "string";
        let_cxx_string!(str = input);
        let _ = Twine::from_cxx_string(&*str);
    }
}

#[test]
fn from_string_ref() {
    unsafe {
        let input = "string";
        let_cxx_string!(str = input);
        let string_ref = StringRef::from_cxx_string(&*str);
        let _ = Twine::from_string_ref(&string_ref);
    }
}
