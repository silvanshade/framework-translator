use clang_importer_sys::{
    llvm::{Triple, Twine},
    swift::LangOptions,
};

#[test]
fn new() {
    unsafe {
        let _ = LangOptions::new();
    }
}

#[test]
fn target() {
    unsafe {
        let opts = LangOptions::new();
        let _ = opts.target();
    }
}

#[test]
fn set_target() {
    unsafe {
        let mut opts = LangOptions::new();
        cxx::let_cxx_string!(arch = "x86_64");
        let arch = Twine::from(&*arch);
        cxx::let_cxx_string!(vendor = "apple");
        let vendor = Twine::from(&*vendor);
        cxx::let_cxx_string!(os = "darwin");
        let os = Twine::from(&*os);
        let triple = Triple::from_arch_vendor_os(&arch, &vendor, &os);
        opts.set_target(triple);
    }
}
