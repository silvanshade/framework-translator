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
        let_cxx_string!(expected = "string");
        assert_eq!(&*expected, &*Twine::from(&*expected).str());
    }
}

#[test]
fn from_string_ref() {
    unsafe {
        let value = "string";
        let_cxx_string!(expected = value);
        assert_eq!(&*expected, &*Twine::from(StringRef::from(value)).str());
    }
}
