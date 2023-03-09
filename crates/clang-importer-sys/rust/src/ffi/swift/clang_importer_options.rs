#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "rust::swift"]
    struct ClangImporterOptions {
        ptr: UniquePtr<CxxClangImporterOptions>,
    }

    #[namespace = "swift"]
    extern "C++" {
        include!("swift/Basic/LangOptions.h");

        #[cxx_name = "ClangImporterOptions"]
        type CxxClangImporterOptions;
    }

    #[namespace = "rust::swift"]
    #[derive(Debug)]
    #[repr(u8)]
    enum Modes {
        Normal,
        EmbedBitcode,
        PrecompiledModule,
    }

    extern "C++" {
        include!("cxx/swift/ClangImporterOptions.hxx");

        type Modes;
    }

    #[namespace = "cxx::swift::ClangImporterOptions"]
    extern "C++" {
        include!("cxx/swift/ClangImporterOptions.hxx");

        unsafe fn make() -> UniquePtr<CxxClangImporterOptions>;

        unsafe fn clangPath(This: &CxxClangImporterOptions) -> &CxxString;

        unsafe fn set_clangPath(This: Pin<&mut CxxClangImporterOptions>, value: &str);

        unsafe fn ModuleCachePath(This: &CxxClangImporterOptions) -> &CxxString;

        unsafe fn set_ModuleCachePath(This: Pin<&mut CxxClangImporterOptions>, value: &str);

        unsafe fn ExtraArgs(This: &CxxClangImporterOptions) -> &CxxVector<CxxString>;

        unsafe fn set_ExtraArgs(This: Pin<&mut CxxClangImporterOptions>, values: &[&str]);

        unsafe fn OverrideResourceDir(This: &CxxClangImporterOptions) -> &CxxString;

        unsafe fn set_OverrideResourceDir(This: Pin<&mut CxxClangImporterOptions>, value: &str);

        unsafe fn TargetCPU(This: &CxxClangImporterOptions) -> &CxxString;

        unsafe fn set_TargetCPU(This: Pin<&mut CxxClangImporterOptions>, value: &str);

        unsafe fn IndexStorePath(This: &CxxClangImporterOptions) -> &CxxString;

        unsafe fn set_IndexStorePath(This: Pin<&mut CxxClangImporterOptions>, value: &str);

        unsafe fn BridgingHeader(This: &CxxClangImporterOptions) -> &CxxString;

        unsafe fn set_BridgingHeader(This: Pin<&mut CxxClangImporterOptions>, value: &str);

        unsafe fn PrecompiledHeaderOutputDir(This: &CxxClangImporterOptions) -> &CxxString;

        unsafe fn set_PrecompiledHeaderOutputDir(This: Pin<&mut CxxClangImporterOptions>, value: &str);

        unsafe fn Optimization(This: &CxxClangImporterOptions) -> &CxxString;

        unsafe fn set_Optimization(This: Pin<&mut CxxClangImporterOptions>, value: &str);

        unsafe fn PCHDisableValidation(This: &CxxClangImporterOptions) -> bool;

        unsafe fn set_PCHDisableValidation(This: Pin<&mut CxxClangImporterOptions>, value: bool);

        unsafe fn Mode(This: &CxxClangImporterOptions) -> Modes;

        unsafe fn set_Mode(This: Pin<&mut CxxClangImporterOptions>, value: Modes);

        unsafe fn DetailedPreprocessingRecord(This: &CxxClangImporterOptions) -> bool;

        unsafe fn set_DetailedPreprocessingRecord(This: Pin<&mut CxxClangImporterOptions>, value: bool);

        unsafe fn DumpClangDiagnostics(This: &CxxClangImporterOptions) -> bool;

        unsafe fn set_DumpClangDiagnostics(This: Pin<&mut CxxClangImporterOptions>, value: bool);

        unsafe fn ImportForwardDeclarations(This: &CxxClangImporterOptions) -> bool;

        unsafe fn set_ImportForwardDeclarations(This: Pin<&mut CxxClangImporterOptions>, value: bool);

        unsafe fn DisableSwiftBridgeAttr(This: &CxxClangImporterOptions) -> bool;

        unsafe fn set_DisableSwiftBridgeAttr(This: Pin<&mut CxxClangImporterOptions>, value: bool);

        unsafe fn DisableOverlayModules(This: &CxxClangImporterOptions) -> bool;

        unsafe fn set_DisableOverlayModules(This: Pin<&mut CxxClangImporterOptions>, value: bool);

        unsafe fn EnableClangSPI(This: &CxxClangImporterOptions) -> bool;

        unsafe fn set_EnableClangSPI(This: Pin<&mut CxxClangImporterOptions>, value: bool);

        unsafe fn DebuggerSupport(This: &CxxClangImporterOptions) -> bool;

        unsafe fn set_DebuggerSupport(This: Pin<&mut CxxClangImporterOptions>, value: bool);

        unsafe fn DisableSourceImport(This: &CxxClangImporterOptions) -> bool;

        unsafe fn set_DisableSourceImport(This: Pin<&mut CxxClangImporterOptions>, value: bool);

        unsafe fn ExtraArgsOnly(This: &CxxClangImporterOptions) -> bool;

        unsafe fn set_ExtraArgsOnly(This: Pin<&mut CxxClangImporterOptions>, value: bool);
    }
}

use self::ffi::{ClangImporterOptions, CxxClangImporterOptions, Modes};
use cxx::{CxxString, UniquePtr};

impl From<UniquePtr<CxxClangImporterOptions>> for ClangImporterOptions {
    #[inline]
    fn from(ptr: UniquePtr<CxxClangImporterOptions>) -> Self {
        Self { ptr }
    }
}

impl ClangImporterOptions {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }

    #[inline]
    pub unsafe fn clang_path(&self) -> &CxxString {
        let this = &self.ptr;
        self::ffi::clangPath(this)
    }

    #[inline]
    pub unsafe fn set_clang_path(&mut self, value: &str) {
        let this = self.ptr.pin_mut();
        self::ffi::set_clangPath(this, value)
    }

    #[inline]
    pub unsafe fn module_cache_path(&self) -> &CxxString {
        let this = &self.ptr;
        self::ffi::ModuleCachePath(this)
    }

    #[inline]
    pub unsafe fn set_module_cache_path(&mut self, value: &str) {
        let this = self.ptr.pin_mut();
        self::ffi::set_ModuleCachePath(this, value)
    }

    #[inline]
    pub unsafe fn extra_args(&self) -> impl Iterator<Item = &CxxString> + '_ {
        let this = &self.ptr;
        let args = self::ffi::ExtraArgs(this);
        args.into_iter()
    }

    #[inline]
    pub unsafe fn set_extra_args(&mut self, values: &[&str]) {
        let this = self.ptr.pin_mut();
        self::ffi::set_ExtraArgs(this, values)
    }

    #[inline]
    pub unsafe fn override_resource_dir(&self) -> &CxxString {
        let this = &self.ptr;
        self::ffi::OverrideResourceDir(this)
    }

    #[inline]
    pub unsafe fn set_override_resource_dir(&mut self, value: &str) {
        let this = self.ptr.pin_mut();
        self::ffi::set_OverrideResourceDir(this, value)
    }

    #[inline]
    pub unsafe fn target_cpu(&self) -> &CxxString {
        let this = &self.ptr;
        self::ffi::TargetCPU(this)
    }

    #[inline]
    pub unsafe fn set_target_cpu(&mut self, value: &str) {
        let this = self.ptr.pin_mut();
        self::ffi::set_TargetCPU(this, value)
    }

    #[inline]
    pub unsafe fn index_store_path(&self) -> &CxxString {
        let this = &self.ptr;
        self::ffi::IndexStorePath(this)
    }

    #[inline]
    pub unsafe fn set_index_store_path(&mut self, value: &str) {
        let this = self.ptr.pin_mut();
        self::ffi::set_IndexStorePath(this, value)
    }

    #[inline]
    pub unsafe fn bridging_header(&self) -> &CxxString {
        let this = &self.ptr;
        self::ffi::BridgingHeader(this)
    }

    #[inline]
    pub unsafe fn set_bridging_header(&mut self, value: &str) {
        let this = self.ptr.pin_mut();
        self::ffi::set_BridgingHeader(this, value)
    }

    #[inline]
    pub unsafe fn precompiled_header_output_dir(&self) -> &CxxString {
        let this = &self.ptr;
        self::ffi::PrecompiledHeaderOutputDir(this)
    }

    #[inline]
    pub unsafe fn set_precompiled_header_output_dir(&mut self, value: &str) {
        let this = self.ptr.pin_mut();
        self::ffi::set_PrecompiledHeaderOutputDir(this, value)
    }

    #[inline]
    pub unsafe fn optimization(&self) -> &CxxString {
        let this = &self.ptr;
        self::ffi::Optimization(this)
    }

    #[inline]
    pub unsafe fn set_optimization(&mut self, value: &str) {
        let this = self.ptr.pin_mut();
        self::ffi::set_Optimization(this, value)
    }

    #[inline]
    pub unsafe fn pch_disable_validation(&self) -> bool {
        let this = &self.ptr;
        self::ffi::PCHDisableValidation(this)
    }

    #[inline]
    pub unsafe fn set_pch_disable_validation(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_PCHDisableValidation(this, value)
    }

    #[inline]
    pub unsafe fn mode(&self) -> Modes {
        let this = &self.ptr;
        self::ffi::Mode(this)
    }

    #[inline]
    pub unsafe fn set_mode(&mut self, value: Modes) {
        let this = self.ptr.pin_mut();
        self::ffi::set_Mode(this, value)
    }

    #[inline]
    pub unsafe fn detailed_preprocessing_record(&self) -> bool {
        let this = &self.ptr;
        self::ffi::DetailedPreprocessingRecord(this)
    }

    #[inline]
    pub unsafe fn set_detailed_preprocessing_record(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DetailedPreprocessingRecord(this, value)
    }

    #[inline]
    pub unsafe fn dump_clang_diagnostics(&self) -> bool {
        let this = &self.ptr;
        self::ffi::DumpClangDiagnostics(this)
    }

    #[inline]
    pub unsafe fn set_dump_clang_diagnostics(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DumpClangDiagnostics(this, value)
    }

    #[inline]
    pub unsafe fn import_forward_declarations(&self) -> bool {
        let this = &self.ptr;
        self::ffi::ImportForwardDeclarations(this)
    }

    #[inline]
    pub unsafe fn set_import_forward_declarations(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_ImportForwardDeclarations(this, value)
    }

    #[inline]
    pub unsafe fn disable_swift_bridge_attr(&self) -> bool {
        let this = &self.ptr;
        self::ffi::DisableSwiftBridgeAttr(this)
    }

    #[inline]
    pub unsafe fn set_disable_swift_bridge_attr(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DisableSwiftBridgeAttr(this, value)
    }

    #[inline]
    pub unsafe fn disable_overlay_modules(&self) -> bool {
        let this = &self.ptr;
        self::ffi::DisableOverlayModules(this)
    }

    #[inline]
    pub unsafe fn set_disable_overlay_modules(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DisableOverlayModules(this, value)
    }

    #[inline]
    pub unsafe fn enable_clang_spi(&self) -> bool {
        let this = &self.ptr;
        self::ffi::EnableClangSPI(this)
    }

    #[inline]
    pub unsafe fn set_enable_clang_spi(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_EnableClangSPI(this, value)
    }

    #[inline]
    pub unsafe fn debugger_support(&self) -> bool {
        let this = &self.ptr;
        self::ffi::DebuggerSupport(this)
    }

    #[inline]
    pub unsafe fn set_debugger_support(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DebuggerSupport(this, value)
    }

    #[inline]
    pub unsafe fn disable_source_import(&self) -> bool {
        let this = &self.ptr;
        self::ffi::DisableSourceImport(this)
    }

    #[inline]
    pub unsafe fn set_disable_source_import(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DisableSourceImport(this, value)
    }

    #[inline]
    pub unsafe fn extra_args_only(&self) -> bool {
        let this = &self.ptr;
        self::ffi::ExtraArgsOnly(this)
    }

    #[inline]
    pub unsafe fn set_extra_args_only(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_ExtraArgsOnly(this, value)
    }
}
