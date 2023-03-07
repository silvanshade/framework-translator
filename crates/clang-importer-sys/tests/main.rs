#![allow(unused)]
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

mod clang;
mod llvm;
mod swift;
