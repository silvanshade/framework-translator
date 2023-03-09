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
        ffi_dir.join(PathBuf::from_iter(["llvm", "hash_code.rs"])),
        ffi_dir.join(PathBuf::from_iter(["llvm", "string_ref.rs"])),
        ffi_dir.join(PathBuf::from_iter(["llvm", "triple.rs"])),
        ffi_dir.join(PathBuf::from_iter(["llvm", "twine.rs"])),
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
    .flag_if_supported("-std=c++20")
    .flag_if_supported("-Werror")
    .flag_if_supported("-Wall")
    .flag_if_supported("-Wextra")
    .flag_if_supported("-pedantic")
    .flag_if_supported("-Wno-ambiguous-reversed-operator")
    .flag_if_supported("-Wno-deprecated-anon-enum-enum-conversion")
    .flag_if_supported("-Wno-deprecated-enum-enum-conversion")
    .flag_if_supported("-Wno-dollar-in-identifier-extension")
    .flag_if_supported("-Wno-unused-parameter")
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
    .try_compile("clang-importer-sys")?;

    let swift_project_dir = PathBuf::from_iter(["..", "swift-project"]);
    let ninja_build_dir = swift_project_dir.join(PathBuf::from_iter(["build", "Ninja-RelWithDebInfoAssert"]));
    let swift_build_dir = ninja_build_dir.join(format!("swift-{NINJA_TARGET}"));
    let llvm_build_dir = ninja_build_dir.join(format!("llvm-{NINJA_TARGET}"));
    let cmark_build_dir = ninja_build_dir.join(format!("cmark-{NINJA_TARGET}"));

    println!("cargo:rustc-link-search={}", swift_build_dir.join("lib").display());
    println!("cargo:rustc-link-search={}", llvm_build_dir.join("lib").display());
    println!("cargo:rustc-link-search={}", cmark_build_dir.join("src").display());

    link_swift_libs()?;
    link_clang_libs()?;
    link_llvm_libs()?;
    link_system_libs()?;

    Ok(())
}

fn link_swift_libs() -> Result<(), BoxError> {
    println!("cargo:rustc-link-lib=static=swiftClangImporter");
    println!("cargo:rustc-link-lib=static=swiftSyntaxParse");
    println!("cargo:rustc-link-lib=static=swiftAST");
    println!("cargo:rustc-link-lib=static=swiftParse");
    println!("cargo:rustc-link-lib=static=swiftMarkup");
    println!("cargo:rustc-link-lib=static=swiftSyntax");
    println!("cargo:rustc-link-lib=static=swiftBasic");
    println!("cargo:rustc-link-lib=static=swiftDemangling");
    Ok(())
}

fn link_clang_libs() -> Result<(), BoxError> {
    println!("cargo:rustc-link-lib=static=clangDependencyScanning");
    println!("cargo:rustc-link-lib=static=clangTooling");
    println!("cargo:rustc-link-lib=static=clangRewriteFrontend");
    println!("cargo:rustc-link-lib=static=clangCodeGen");
    println!("cargo:rustc-link-lib=static=clangIndex");
    println!("cargo:rustc-link-lib=static=clangFrontend");
    println!("cargo:rustc-link-lib=static=clangDriver");
    println!("cargo:rustc-link-lib=static=clangParse");
    println!("cargo:rustc-link-lib=static=clangSerialization");
    println!("cargo:rustc-link-lib=static=clangSema");
    println!("cargo:rustc-link-lib=static=clangAPINotes");
    println!("cargo:rustc-link-lib=static=clangEdit");
    println!("cargo:rustc-link-lib=static=clangAnalysis");
    println!("cargo:rustc-link-lib=static=clangASTMatchers");
    println!("cargo:rustc-link-lib=static=clangLex");
    println!("cargo:rustc-link-lib=static=clangAST");
    println!("cargo:rustc-link-lib=static=clangBasic");
    Ok(())
}

fn link_llvm_libs() -> Result<(), BoxError> {
    println!("cargo:rustc-link-lib=static=LLVMOption");
    println!("cargo:rustc-link-lib=static=LLVMAArch64AsmParser");
    println!("cargo:rustc-link-lib=static=LLVMAArch64CodeGen");
    println!("cargo:rustc-link-lib=static=LLVMAArch64Utils");
    println!("cargo:rustc-link-lib=static=LLVMAArch64Desc");
    println!("cargo:rustc-link-lib=static=LLVMAArch64Info");
    println!("cargo:rustc-link-lib=static=LLVMARMAsmParser");
    println!("cargo:rustc-link-lib=static=LLVMARMCodeGen");
    println!("cargo:rustc-link-lib=static=LLVMARMUtils");
    println!("cargo:rustc-link-lib=static=LLVMARMDesc");
    println!("cargo:rustc-link-lib=static=LLVMARMInfo");
    println!("cargo:rustc-link-lib=static=LLVMMipsAsmParser");
    println!("cargo:rustc-link-lib=static=LLVMMipsCodeGen");
    println!("cargo:rustc-link-lib=static=LLVMMipsDesc");
    println!("cargo:rustc-link-lib=static=LLVMMipsInfo");
    println!("cargo:rustc-link-lib=static=LLVMPowerPCAsmParser");
    println!("cargo:rustc-link-lib=static=LLVMPowerPCCodeGen");
    println!("cargo:rustc-link-lib=static=LLVMPowerPCDesc");
    println!("cargo:rustc-link-lib=static=LLVMPowerPCInfo");
    println!("cargo:rustc-link-lib=static=LLVMSystemZAsmParser");
    println!("cargo:rustc-link-lib=static=LLVMSystemZCodeGen");
    println!("cargo:rustc-link-lib=static=LLVMSystemZDesc");
    println!("cargo:rustc-link-lib=static=LLVMSystemZInfo");
    println!("cargo:rustc-link-lib=static=LLVMX86AsmParser");
    println!("cargo:rustc-link-lib=static=LLVMX86CodeGen");
    println!("cargo:rustc-link-lib=static=LLVMX86Desc");
    println!("cargo:rustc-link-lib=static=LLVMX86Info");
    println!("cargo:rustc-link-lib=static=LLVMMCDisassembler");
    println!("cargo:rustc-link-lib=static=LLVMMCParser");
    println!("cargo:rustc-link-lib=static=LLVMAsmPrinter");
    println!("cargo:rustc-link-lib=static=LLVMDebugInfoCodeView");
    println!("cargo:rustc-link-lib=static=LLVMDebugInfoDWARF");
    println!("cargo:rustc-link-lib=static=LLVMCFGuard");
    println!("cargo:rustc-link-lib=static=LLVMGlobalISel");
    println!("cargo:rustc-link-lib=static=LLVMSelectionDAG");
    println!("cargo:rustc-link-lib=static=LLVMCodeGen");
    println!("cargo:rustc-link-lib=static=LLVMTarget");
    println!("cargo:rustc-link-lib=static=LLVMLTO");
    println!("cargo:rustc-link-lib=static=LLVMLinker");
    println!("cargo:rustc-link-lib=static=LLVMPasses");
    println!("cargo:rustc-link-lib=static=LLVMObjCARCOpts");
    println!("cargo:rustc-link-lib=static=LLVMAggressiveInstCombine");
    println!("cargo:rustc-link-lib=static=LLVMCoroutines");
    println!("cargo:rustc-link-lib=static=LLVMInstrumentation");
    println!("cargo:rustc-link-lib=static=LLVMInstCombine");
    println!("cargo:rustc-link-lib=static=LLVMVectorize");
    println!("cargo:rustc-link-lib=static=LLVMipo");
    println!("cargo:rustc-link-lib=static=LLVMIRReader");
    println!("cargo:rustc-link-lib=static=LLVMAsmParser");
    println!("cargo:rustc-link-lib=static=LLVMBitWriter");
    println!("cargo:rustc-link-lib=static=LLVMCoverage");
    println!("cargo:rustc-link-lib=static=LLVMProfileData");
    println!("cargo:rustc-link-lib=static=LLVMBitstreamReader");
    println!("cargo:rustc-link-lib=static=LLVMFrontendOpenMP");
    println!("cargo:rustc-link-lib=static=LLVMScalarOpts");
    println!("cargo:rustc-link-lib=static=LLVMMC");
    println!("cargo:rustc-link-lib=static=LLVMTransformUtils");
    println!("cargo:rustc-link-lib=static=LLVMAnalysis");
    println!("cargo:rustc-link-lib=static=LLVMObject");
    println!("cargo:rustc-link-lib=static=LLVMTextAPI");
    println!("cargo:rustc-link-lib=static=LLVMBitReader");
    println!("cargo:rustc-link-lib=static=LLVMCore");
    println!("cargo:rustc-link-lib=static=LLVMBinaryFormat");
    println!("cargo:rustc-link-lib=static=LLVMRemarks");
    println!("cargo:rustc-link-lib=static=LLVMSupport");
    println!("cargo:rustc-link-lib=static=LLVMDemangle");
    Ok(())
}

fn link_system_libs() -> Result<(), BoxError> {
    println!("cargo:rustc-link-lib=static=cmark");
    println!("cargo:rustc-link-lib=ncurses");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=z");
    #[cfg(target_os = "macos")]
    {
        let stdout = std::process::Command::new("clang").arg("--version").output()?.stdout;
        let stdout = std::str::from_utf8(&stdout)?;
        if stdout.starts_with("Apple clang version") {
            if let Some(clang_version) = stdout.split_whitespace().nth(3) {
                #[rustfmt::skip]
                println!("cargo:rustc-link-search=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/{clang_version}/lib/darwin");
                println!("cargo:rustc-link-lib=static=clang_rt.osx");
            }
        } else {
            return Err("Expected `Apple clang version` string from `clang --version`".into());
        }
    }
    Ok(())
}
