#pragma once

#include "rust/cxx.h"
#include "swift/Basic/LangOptions.h"

#include "llvm/ADT/StringRef.h"

#include <memory>

// NOTE: these are global since cxx emits enum asserts without qualified names
using Modes = ::swift::ClangImporterOptions::Modes;

namespace cxx {
namespace swift {
namespace ClangImporterOptions {

[[gnu::always_inline]] static inline std::unique_ptr<::swift::ClangImporterOptions>
make()
{
  return std::make_unique<::swift::ClangImporterOptions>(::swift::ClangImporterOptions());
}

[[gnu::always_inline]] static inline std::string const&
clangPath(::swift::ClangImporterOptions const& This)
{
  return This.clangPath;
}

[[gnu::always_inline]] static inline void
set_clangPath(::swift::ClangImporterOptions& This, rust::Str value)
{
  This.clangPath = std::string(value);
}

[[gnu::always_inline]] static inline std::string const&
ModuleCachePath(::swift::ClangImporterOptions const& This)
{
  return This.ModuleCachePath;
}

[[gnu::always_inline]] static inline void
set_ModuleCachePath(::swift::ClangImporterOptions& This, rust::Str value)
{
  This.ModuleCachePath = std::string(value);
}

[[gnu::always_inline]] static inline std::vector<std::string> const&
ExtraArgs(::swift::ClangImporterOptions const& This)
{
  return This.ExtraArgs;
}

[[gnu::always_inline]] static inline void
set_ExtraArgs(::swift::ClangImporterOptions& This, rust::Slice<rust::Str const> that)
{
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
    This.ExtraArgs.erase(This.ExtraArgs.begin() + thatSize, This.ExtraArgs.end());
  } else {
    // subsequently extend with additional
    for (; i != thatSize; ++i) {
      This.ExtraArgs.emplace_back(that[i]);
    }
  }
}

[[gnu::always_inline]] static inline std::string const&
OverrideResourceDir(::swift::ClangImporterOptions const& This)
{
  return This.OverrideResourceDir;
}

[[gnu::always_inline]] static inline void
set_OverrideResourceDir(::swift::ClangImporterOptions& This, rust::Str value)
{
  This.OverrideResourceDir = std::string(value);
}

[[gnu::always_inline]] static inline std::string const&
TargetCPU(::swift::ClangImporterOptions const& This)
{
  return This.TargetCPU;
}

[[gnu::always_inline]] static inline void
set_TargetCPU(::swift::ClangImporterOptions& This, rust::Str value)
{
  This.TargetCPU = std::string(value);
}

[[gnu::always_inline]] static inline std::string const&
IndexStorePath(::swift::ClangImporterOptions const& This)
{
  return This.IndexStorePath;
}

[[gnu::always_inline]] static inline void
set_IndexStorePath(::swift::ClangImporterOptions& This, rust::Str value)
{
  This.IndexStorePath = std::string(value);
}

[[gnu::always_inline]] static inline std::string const&
BridgingHeader(::swift::ClangImporterOptions const& This)
{
  return This.BridgingHeader;
}

[[gnu::always_inline]] static inline void
set_BridgingHeader(::swift::ClangImporterOptions& This, rust::Str value)
{
  This.BridgingHeader = std::string(value);
}

[[gnu::always_inline]] static inline std::string const&
PrecompiledHeaderOutputDir(::swift::ClangImporterOptions const& This)
{
  return This.PrecompiledHeaderOutputDir;
}

[[gnu::always_inline]] static inline void
set_PrecompiledHeaderOutputDir(::swift::ClangImporterOptions& This, rust::Str value)
{
  This.PrecompiledHeaderOutputDir = std::string(value);
}

[[gnu::always_inline]] static inline std::string const&
Optimization(::swift::ClangImporterOptions const& This)
{
  return This.Optimization;
}

[[gnu::always_inline]] static inline void
set_Optimization(::swift::ClangImporterOptions& This, rust::Str value)
{
  This.Optimization = std::string(value);
}

[[gnu::always_inline]] static inline bool
PCHDisableValidation(::swift::ClangImporterOptions const& This)
{
  return This.PCHDisableValidation;
}

[[gnu::always_inline]] static inline void
set_PCHDisableValidation(::swift::ClangImporterOptions& This, bool value)
{
  This.PCHDisableValidation = value;
}

[[gnu::always_inline]] static inline ::swift::ClangImporterOptions::Modes
Mode(::swift::ClangImporterOptions const& This)
{
  return This.Mode;
}

[[gnu::always_inline]] static inline void
set_Mode(::swift::ClangImporterOptions& This, ::swift::ClangImporterOptions::Modes value)
{
  This.Mode = value;
}

[[gnu::always_inline]] static inline bool
DetailedPreprocessingRecord(::swift::ClangImporterOptions const& This)
{
  return This.DetailedPreprocessingRecord;
}

[[gnu::always_inline]] static inline void
set_DetailedPreprocessingRecord(::swift::ClangImporterOptions& This, bool value)
{
  This.DetailedPreprocessingRecord = value;
}

[[gnu::always_inline]] static inline bool
DumpClangDiagnostics(::swift::ClangImporterOptions const& This)
{
  return This.DumpClangDiagnostics;
}

[[gnu::always_inline]] static inline void
set_DumpClangDiagnostics(::swift::ClangImporterOptions& This, bool value)
{
  This.DumpClangDiagnostics = value;
}

[[gnu::always_inline]] static inline bool
ImportForwardDeclarations(::swift::ClangImporterOptions const& This)
{
  return This.ImportForwardDeclarations;
}

[[gnu::always_inline]] static inline void
set_ImportForwardDeclarations(::swift::ClangImporterOptions& This, bool value)
{
  This.ImportForwardDeclarations = value;
}

[[gnu::always_inline]] static inline bool
DisableSwiftBridgeAttr(::swift::ClangImporterOptions const& This)
{
  return This.DisableSwiftBridgeAttr;
}

[[gnu::always_inline]] static inline void
set_DisableSwiftBridgeAttr(::swift::ClangImporterOptions& This, bool value)
{
  This.DisableSwiftBridgeAttr = value;
}

[[gnu::always_inline]] static inline bool
DisableOverlayModules(::swift::ClangImporterOptions const& This)
{
  return This.DisableOverlayModules;
}

[[gnu::always_inline]] static inline void
set_DisableOverlayModules(::swift::ClangImporterOptions& This, bool value)
{
  This.DisableOverlayModules = value;
}

[[gnu::always_inline]] static inline bool
EnableClangSPI(::swift::ClangImporterOptions const& This)
{
  return This.EnableClangSPI;
}

[[gnu::always_inline]] static inline void
set_EnableClangSPI(::swift::ClangImporterOptions& This, bool value)
{
  This.EnableClangSPI = value;
}

[[gnu::always_inline]] static inline bool
DebuggerSupport(::swift::ClangImporterOptions const& This)
{
  return This.DebuggerSupport;
}

[[gnu::always_inline]] static inline void
set_DebuggerSupport(::swift::ClangImporterOptions& This, bool value)
{
  This.DebuggerSupport = value;
}

[[gnu::always_inline]] static inline bool
DisableSourceImport(::swift::ClangImporterOptions const& This)
{
  return This.DisableSourceImport;
}

[[gnu::always_inline]] static inline void
set_DisableSourceImport(::swift::ClangImporterOptions& This, bool value)
{
  This.DisableSourceImport = value;
}

[[gnu::always_inline]] static inline bool
ExtraArgsOnly(::swift::ClangImporterOptions const& This)
{
  return This.ExtraArgsOnly;
}

[[gnu::always_inline]] static inline void
set_ExtraArgsOnly(::swift::ClangImporterOptions& This, bool value)
{
  This.ExtraArgsOnly = value;
}

[[gnu::always_inline]] static inline std::unique_ptr<::llvm::hash_code>
getPCHHashComponents(::swift::ClangImporterOptions const& This)
{
  return std::make_unique<::llvm::hash_code>(This.getPCHHashComponents());
}

[[gnu::always_inline]] static inline std::unique_ptr<std::vector<std::string>>
getRemappedExtraArgs(::swift::ClangImporterOptions const& This, rust::Fn<rust::String(rust::Str)> pathRemapCallback)
{
  auto lambda = [=](::llvm::StringRef s) {
    auto string = (*pathRemapCallback)(rust::Str(s.data(), s.size()));
    return std::string(string.data(), string.size());
  };
  return std::make_unique<std::vector<std::string>>(This.getRemappedExtraArgs(lambda));
}

} // namespace ClangImporterOptions
} // namespace swift
} // namespace cxx
