#include "cxx/swift/ClangImporterOptions.hxx"

namespace cxx {
namespace swift {
namespace ClangImporterOptions {

std::unique_ptr<::swift::ClangImporterOptions> make() {
  return std::make_unique<::swift::ClangImporterOptions>(
      ::swift::ClangImporterOptions());
}

std::string const &clangPath(::swift::ClangImporterOptions const &This) {
  return This.clangPath;
}

void set_clangPath(::swift::ClangImporterOptions &This, rust::Str value) {
  This.clangPath = std::string(value);
}

std::string const &ModuleCachePath(::swift::ClangImporterOptions const &This) {
  return This.ModuleCachePath;
}

void set_ModuleCachePath(::swift::ClangImporterOptions &This, rust::Str value) {
  This.ModuleCachePath = std::string(value);
}

std::vector<std::string> const &
ExtraArgs(::swift::ClangImporterOptions const &This) {
  return This.ExtraArgs;
}

void set_ExtraArgs(::swift::ClangImporterOptions &This,
                   rust::Slice<rust::Str const> that) {
  auto thisSize = This.ExtraArgs.size();
  auto thatSize = that.size();
  This.ExtraArgs.reserve(thatSize);
  std::size_t i = 0;
  // overwrite initial existing
  for (; i != thatSize && i != thisSize; ++i) {
    This.ExtraArgs[i] = std::string(that[i]);
  }
  if (thatSize < thisSize) {
    // erase trailing
    This.ExtraArgs.erase(This.ExtraArgs.begin() + thatSize,
                         This.ExtraArgs.end());
  } else {
    // subsequently extend with additional
    for (; i != thatSize; ++i) {
      This.ExtraArgs.emplace_back(that[i]);
    }
  }
}

std::string const &
OverrideResourceDir(::swift::ClangImporterOptions const &This) {
  return This.OverrideResourceDir;
}

void set_OverrideResourceDir(::swift::ClangImporterOptions &This,
                             rust::Str value) {
  This.OverrideResourceDir = std::string(value);
}

std::string const &TargetCPU(::swift::ClangImporterOptions const &This) {
  return This.TargetCPU;
}

void set_TargetCPU(::swift::ClangImporterOptions &This, rust::Str value) {
  This.TargetCPU = std::string(value);
}

std::string const &IndexStorePath(::swift::ClangImporterOptions const &This) {
  return This.IndexStorePath;
}

void set_IndexStorePath(::swift::ClangImporterOptions &This, rust::Str value) {
  This.IndexStorePath = std::string(value);
}

std::string const &BridgingHeader(::swift::ClangImporterOptions const &This) {
  return This.BridgingHeader;
}

void set_BridgingHeader(::swift::ClangImporterOptions &This, rust::Str value) {
  This.BridgingHeader = std::string(value);
}

std::string const &
PrecompiledHeaderOutputDir(::swift::ClangImporterOptions const &This) {
  return This.PrecompiledHeaderOutputDir;
}

void set_PrecompiledHeaderOutputDir(::swift::ClangImporterOptions &This,
                                    rust::Str value) {
  This.PrecompiledHeaderOutputDir = std::string(value);
}

std::string const &Optimization(::swift::ClangImporterOptions const &This) {
  return This.Optimization;
}

void set_Optimization(::swift::ClangImporterOptions &This, rust::Str value) {
  This.Optimization = std::string(value);
}

bool PCHDisableValidation(::swift::ClangImporterOptions const &This) {
  return This.PCHDisableValidation;
}

void set_PCHDisableValidation(::swift::ClangImporterOptions &This, bool value) {
  This.PCHDisableValidation = value;
}

Modes Mode(::swift::ClangImporterOptions const &This) { return This.Mode; }

void set_Mode(::swift::ClangImporterOptions &This, Modes value) {
  This.Mode = value;
}

bool DetailedPreprocessingRecord(::swift::ClangImporterOptions const &This) {
  return This.DetailedPreprocessingRecord;
}

void set_DetailedPreprocessingRecord(::swift::ClangImporterOptions &This,
                                     bool value) {
  This.DetailedPreprocessingRecord = value;
}

bool DumpClangDiagnostics(::swift::ClangImporterOptions const &This) {
  return This.DumpClangDiagnostics;
}

void set_DumpClangDiagnostics(::swift::ClangImporterOptions &This, bool value) {
  This.DumpClangDiagnostics = value;
}

bool ImportForwardDeclarations(::swift::ClangImporterOptions const &This) {
  return This.ImportForwardDeclarations;
}

void set_ImportForwardDeclarations(::swift::ClangImporterOptions &This,
                                   bool value) {
  This.ImportForwardDeclarations = value;
}

bool DisableSwiftBridgeAttr(::swift::ClangImporterOptions const &This) {
  return This.DisableSwiftBridgeAttr;
}

void set_DisableSwiftBridgeAttr(::swift::ClangImporterOptions &This,
                                bool value) {
  This.DisableSwiftBridgeAttr = value;
}

bool DisableOverlayModules(::swift::ClangImporterOptions const &This) {
  return This.DisableOverlayModules;
}

void set_DisableOverlayModules(::swift::ClangImporterOptions &This,
                               bool value) {
  This.DisableOverlayModules = value;
}

bool EnableClangSPI(::swift::ClangImporterOptions const &This) {
  return This.EnableClangSPI;
}

void set_EnableClangSPI(::swift::ClangImporterOptions &This, bool value) {
  This.EnableClangSPI = value;
}

bool DebuggerSupport(::swift::ClangImporterOptions const &This) {
  return This.DebuggerSupport;
}

void set_DebuggerSupport(::swift::ClangImporterOptions &This, bool value) {
  This.DebuggerSupport = value;
}

bool DisableSourceImport(::swift::ClangImporterOptions const &This) {
  return This.DisableSourceImport;
}

void set_DisableSourceImport(::swift::ClangImporterOptions &This, bool value) {
  This.DisableSourceImport = value;
}

bool ExtraArgsOnly(::swift::ClangImporterOptions const &This) {
  return This.ExtraArgsOnly;
}

void set_ExtraArgsOnly(::swift::ClangImporterOptions &This, bool value) {
  This.ExtraArgsOnly = value;
}

} // namespace ClangImporterOptions
} // namespace swift
} // namespace cxx
