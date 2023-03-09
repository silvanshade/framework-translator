#[cxx::bridge(namespace = "swift")]
pub(crate) mod ffi {
    #[derive(Debug)]
    #[repr(u8)]
    enum FunctionBodySkipping {
        None,
        NonInlinable,
        NonInlinableWithoutTypes,
        All,
    }

    extern "C++" {
        include!("cxx/swift/FunctionBodySkipping.hxx");

        type FunctionBodySkipping;
    }
}
