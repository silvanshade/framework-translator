use crate::BoxError;
use clang_importer_sys::llvm::StringRef;
use cxx::let_cxx_string;
use tap::prelude::*;

#[test]
fn new() {
    let _ = StringRef::new();
}

#[test]
fn from_cxx_string() {
    let input = "string";
    let_cxx_string!(str = input);
    let value = StringRef::from(&*str);
    let ptr = value.data();
    let output = unsafe { std::ffi::CStr::from_ptr(ptr) }.to_string_lossy();
    assert_eq!(output, input);
}

#[test]
fn data() {
    let input = "string";
    let_cxx_string!(str = input);
    let value = StringRef::from(&*str);
    let ptr = value.data();
    let output = unsafe { std::ffi::CStr::from_ptr(ptr) }.to_string_lossy();
    assert_eq!(output, input);
}

#[test]
fn empty() {
    let value = StringRef::new();
    assert!(value.empty());
}

#[test]
fn size_0() {
    let value = StringRef::new();
    assert_eq!(0, value.size());
}

#[test]
fn size_n() {
    let input = "string";
    let_cxx_string!(str = input);
    let value = StringRef::from(&*str);
    assert_eq!(input.len(), value.size());
}

#[test]
fn front() -> Result<(), BoxError> {
    let input = "string";
    let_cxx_string!(str = input);
    let value = StringRef::from(&*str);
    assert_eq!(
        input.chars().nth(0),
        value.front().pipe(u8::try_from).map(Into::into).ok()
    );
    Ok(())
}

#[test]
fn back() -> Result<(), BoxError> {
    let input = "string";
    let_cxx_string!(str = input);
    let value = StringRef::from(&*str);
    assert_eq!(
        input.chars().nth(input.len() - 1),
        value.back().pipe(u8::try_from).map(Into::into).ok()
    );
    Ok(())
}

#[test]
fn equals() {
    let input0 = "string";
    let_cxx_string!(str0 = input0);
    let value0 = StringRef::from(&*str0);
    let input1 = "string";
    let_cxx_string!(str1 = input1);
    let value1 = StringRef::from(&*str1);
    assert!(value0.equals(&value1));
}

#[test]
fn not_equals() {
    let input0 = "lhs";
    let_cxx_string!(str0 = input0);
    let value0 = StringRef::from(&*str0);
    let input1 = "rhs";
    let_cxx_string!(str1 = input1);
    let value1 = StringRef::from(&*str1);
    assert!(!value0.equals(&value1));
}

#[test]
fn equals_insensitive() {
    let input0 = "string";
    let_cxx_string!(str0 = input0);
    let value0 = StringRef::from(&*str0);
    let input1 = "STRING";
    let_cxx_string!(str1 = input1);
    let value1 = StringRef::from(&*str1);
    assert!(value0.equals_insensitive(&value1));
}

#[test]
fn not_equals_insensitive() {
    let input0 = "lhs";
    let_cxx_string!(str0 = input0);
    let value0 = StringRef::from(&*str0);
    let input1 = "rhs";
    let_cxx_string!(str1 = input1);
    let value1 = StringRef::from(&*str1);
    assert!(!value0.equals_insensitive(&value1));
}

#[test]
fn count() {
    let input = "example";
    let_cxx_string!(str = input);
    let value = StringRef::from(&*str);
    assert_eq!(2, value.count('e' as i8));
}
