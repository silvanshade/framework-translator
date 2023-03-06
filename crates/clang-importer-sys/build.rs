use std::path::PathBuf;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const NINJA_TARGET: &str = "macosx-x86_64";
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const NINJA_TARGET: &str = "macosx-arm64";
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const NINJA_TARGET: &str = "linux-x86_64";
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
const NINJA_TARGET: &str = "linux_aarch64";

fn main() -> Result<(), BoxError> {
    println!("cargo:rerun-if-changed=cxx");
    println!("cargo:rerun-if-changed=rust");

    let swift_project_dir = PathBuf::from_iter(["..", "..", "..", "swift-project"]);
    let llvm_project_dir = swift_project_dir.join("llvm-project");

    let ninja_build_dir = swift_project_dir.join(PathBuf::from_iter(["build", "Ninja-RelWithDebInfoAssert"]));
    let swift_build_dir = ninja_build_dir.join(format!("swift-{NINJA_TARGET}"));
    let llvm_build_dir = ninja_build_dir.join(format!("llvm-{NINJA_TARGET}"));
    let clang_build_dir = llvm_build_dir.join(PathBuf::from_iter(["tools", "clang"]));

    let cargo_target_dir = PathBuf::from_iter(["..", "..", "target"]);
    let cxxbridge_dir = cargo_target_dir.join("cxxbridge");
    let ffi_dir = PathBuf::from_iter(["rust", "src", "ffi"]);

    cxx_build::bridges([
        ffi_dir.join(PathBuf::from_iter(["llvm", "string_ref.rs"])),
        ffi_dir.join(PathBuf::from_iter(["swift", "ast_context.rs"])),
        ffi_dir.join(PathBuf::from_iter(["swift", "clang_importer_options.rs"])),
        ffi_dir.join(PathBuf::from_iter(["swift", "clang_importer.rs"])),
        ffi_dir.join(PathBuf::from_iter(["swift", "diagnostic_engine.rs"])),
        ffi_dir.join(PathBuf::from_iter(["swift", "lang_options.rs"])),
        ffi_dir.join(PathBuf::from_iter(["swift", "search_path_options.rs"])),
        ffi_dir.join(PathBuf::from_iter(["swift", "sil_options.rs"])),
        ffi_dir.join(PathBuf::from_iter(["swift", "source_manager.rs"])),
        ffi_dir.join(PathBuf::from_iter([
            "swift",
            "symbolgraphgen",
            "symbol_graph_options.rs",
        ])),
        ffi_dir.join(PathBuf::from_iter(["swift", "type_checker_options.rs"])),
    ])
    .compiler("clang++")
    .includes([
        cxxbridge_dir.join(PathBuf::from_iter(["clang-importer-sys", "rust", "src"])),
        cxxbridge_dir,
        PathBuf::from_iter(["cxx", "include"]),
        swift_build_dir.join("include"),
        clang_build_dir.join("include"),
        llvm_build_dir.join("include"),
        swift_project_dir.join(PathBuf::from_iter(["swift", "include"])),
        llvm_project_dir.join(PathBuf::from_iter(["clang", "include"])),
        llvm_project_dir.join(PathBuf::from_iter(["llvm", "include"])),
    ])
    .files([
        PathBuf::from_iter(["cxx", "lib", "cxx", "llvm", "StringRef.cxx"]),
        PathBuf::from_iter(["cxx", "lib", "cxx", "swift", "symbolgraphgen", "SymbolGraphGen.cxx"]),
        PathBuf::from_iter(["cxx", "lib", "cxx", "swift", "ASTContext.cxx"]),
        PathBuf::from_iter(["cxx", "lib", "cxx", "swift", "ClangImporter.cxx"]),
        PathBuf::from_iter(["cxx", "lib", "cxx", "swift", "ClangImporterOptions.cxx"]),
        PathBuf::from_iter(["cxx", "lib", "cxx", "swift", "DiagnosticEngine.cxx"]),
        PathBuf::from_iter(["cxx", "lib", "cxx", "swift", "LangOptions.cxx"]),
        PathBuf::from_iter(["cxx", "lib", "cxx", "swift", "SearchPathOptions.cxx"]),
        PathBuf::from_iter(["cxx", "lib", "cxx", "swift", "SILOptions.cxx"]),
        PathBuf::from_iter(["cxx", "lib", "cxx", "swift", "SourceManager.cxx"]),
        PathBuf::from_iter(["cxx", "lib", "cxx", "swift", "TypeCheckerOptions.cxx"]),
    ])
    .flag_if_supported("-std=c++20")
    .flag_if_supported("-Wno-ambiguous-reversed-operator")
    .flag_if_supported("-Wno-deprecated-anon-enum-enum-conversion")
    .flag_if_supported("-Wno-deprecated-enum-enum-conversion")
    .flag_if_supported("-Wno-unused-parameter")
    .try_compile("clang-importer-sys")?;

    let swift_project_dir = PathBuf::from_iter(["..", "swift-project"]);
    let ninja_build_dir = swift_project_dir.join(PathBuf::from_iter(["build", "Ninja-RelWithDebInfoAssert"]));
    let swift_build_dir = ninja_build_dir.join(format!("swift-{NINJA_TARGET}"));
    let llvm_build_dir = ninja_build_dir.join(format!("llvm-{NINJA_TARGET}"));
    let cmark_build_dir = ninja_build_dir.join(format!("cmark-{NINJA_TARGET}"));

    println!("cargo:rustc-link-search={}", swift_build_dir.join("lib").display());
    println!("cargo:rustc-link-search={}", llvm_build_dir.join("lib").display());
    println!("cargo:rustc-link-search={}", cmark_build_dir.join("src").display());

    link_swift_libs();
    link_clang_libs();
    link_llvm_libs();
    link_system_libs();

    Ok(())
}

fn link_swift_libs() {
    println!("cargo:rustc-link-lib=swiftClangImporter");
    println!("cargo:rustc-link-lib=swiftSyntaxParse");
    println!("cargo:rustc-link-lib=swiftAST");
    println!("cargo:rustc-link-lib=swiftParse");
    println!("cargo:rustc-link-lib=swiftMarkup");
    println!("cargo:rustc-link-lib=swiftSyntax");
    println!("cargo:rustc-link-lib=swiftBasic");
    println!("cargo:rustc-link-lib=swiftDemangling");
}

fn link_clang_libs() {
    println!("cargo:rustc-link-lib=clangDependencyScanning");
    println!("cargo:rustc-link-lib=clangTooling");
    println!("cargo:rustc-link-lib=clangRewriteFrontend");
    println!("cargo:rustc-link-lib=clangCodeGen");
    println!("cargo:rustc-link-lib=clangIndex");
    println!("cargo:rustc-link-lib=clangFrontend");
    println!("cargo:rustc-link-lib=clangDriver");
    println!("cargo:rustc-link-lib=clangParse");
    println!("cargo:rustc-link-lib=clangSerialization");
    println!("cargo:rustc-link-lib=clangSema");
    println!("cargo:rustc-link-lib=clangAPINotes");
    println!("cargo:rustc-link-lib=clangEdit");
    println!("cargo:rustc-link-lib=clangAnalysis");
    println!("cargo:rustc-link-lib=clangLex");
    println!("cargo:rustc-link-lib=clangAST");
    println!("cargo:rustc-link-lib=clangBasic");
}

fn link_llvm_libs() {
    println!("cargo:rustc-link-lib=LLVMOption");
    println!("cargo:rustc-link-lib=LLVMAArch64AsmParser");
    println!("cargo:rustc-link-lib=LLVMAArch64CodeGen");
    println!("cargo:rustc-link-lib=LLVMAArch64Utils");
    println!("cargo:rustc-link-lib=LLVMAArch64Desc");
    println!("cargo:rustc-link-lib=LLVMAArch64Info");
    println!("cargo:rustc-link-lib=LLVMARMAsmParser");
    println!("cargo:rustc-link-lib=LLVMARMCodeGen");
    println!("cargo:rustc-link-lib=LLVMARMUtils");
    println!("cargo:rustc-link-lib=LLVMARMDesc");
    println!("cargo:rustc-link-lib=LLVMARMInfo");
    println!("cargo:rustc-link-lib=LLVMMipsAsmParser");
    println!("cargo:rustc-link-lib=LLVMMipsCodeGen");
    println!("cargo:rustc-link-lib=LLVMMipsDesc");
    println!("cargo:rustc-link-lib=LLVMMipsInfo");
    println!("cargo:rustc-link-lib=LLVMPowerPCAsmParser");
    println!("cargo:rustc-link-lib=LLVMPowerPCCodeGen");
    println!("cargo:rustc-link-lib=LLVMPowerPCDesc");
    println!("cargo:rustc-link-lib=LLVMPowerPCInfo");
    println!("cargo:rustc-link-lib=LLVMSystemZAsmParser");
    println!("cargo:rustc-link-lib=LLVMSystemZCodeGen");
    println!("cargo:rustc-link-lib=LLVMSystemZDesc");
    println!("cargo:rustc-link-lib=LLVMSystemZInfo");
    println!("cargo:rustc-link-lib=LLVMX86AsmParser");
    println!("cargo:rustc-link-lib=LLVMX86CodeGen");
    println!("cargo:rustc-link-lib=LLVMX86Desc");
    println!("cargo:rustc-link-lib=LLVMX86Info");
    println!("cargo:rustc-link-lib=LLVMMCDisassembler");
    println!("cargo:rustc-link-lib=LLVMMCParser");
    println!("cargo:rustc-link-lib=LLVMAsmPrinter");
    println!("cargo:rustc-link-lib=LLVMDebugInfoCodeView");
    println!("cargo:rustc-link-lib=LLVMDebugInfoDWARF");
    println!("cargo:rustc-link-lib=LLVMCFGuard");
    println!("cargo:rustc-link-lib=LLVMGlobalISel");
    println!("cargo:rustc-link-lib=LLVMSelectionDAG");
    println!("cargo:rustc-link-lib=LLVMCodeGen");
    println!("cargo:rustc-link-lib=LLVMTarget");
    println!("cargo:rustc-link-lib=LLVMLTO");
    println!("cargo:rustc-link-lib=LLVMLinker");
    println!("cargo:rustc-link-lib=LLVMPasses");
    println!("cargo:rustc-link-lib=LLVMObjCARCOpts");
    println!("cargo:rustc-link-lib=LLVMAggressiveInstCombine");
    println!("cargo:rustc-link-lib=LLVMCoroutines");
    println!("cargo:rustc-link-lib=LLVMInstrumentation");
    println!("cargo:rustc-link-lib=LLVMInstCombine");
    println!("cargo:rustc-link-lib=LLVMVectorize");
    println!("cargo:rustc-link-lib=LLVMipo");
    println!("cargo:rustc-link-lib=LLVMIRReader");
    println!("cargo:rustc-link-lib=LLVMAsmParser");
    println!("cargo:rustc-link-lib=LLVMBitWriter");
    println!("cargo:rustc-link-lib=LLVMCoverage");
    println!("cargo:rustc-link-lib=LLVMProfileData");
    println!("cargo:rustc-link-lib=LLVMBitstreamReader");
    println!("cargo:rustc-link-lib=LLVMFrontendOpenMP");
    println!("cargo:rustc-link-lib=LLVMScalarOpts");
    println!("cargo:rustc-link-lib=LLVMMC");
    println!("cargo:rustc-link-lib=LLVMTransformUtils");
    println!("cargo:rustc-link-lib=LLVMAnalysis");
    println!("cargo:rustc-link-lib=LLVMObject");
    println!("cargo:rustc-link-lib=LLVMTextAPI");
    println!("cargo:rustc-link-lib=LLVMBitReader");
    println!("cargo:rustc-link-lib=LLVMCore");
    println!("cargo:rustc-link-lib=LLVMBinaryFormat");
    println!("cargo:rustc-link-lib=LLVMRemarks");
    println!("cargo:rustc-link-lib=LLVMSupport");
    println!("cargo:rustc-link-lib=LLVMDemangle");
}

fn link_system_libs() {
    println!("cargo:rustc-link-arg=-l:libcmark.a");
    println!("cargo:rustc-link-lib=ncurses");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=z");
}
