#pragma once

#include "rust/cxx.h"
#include "swift/Basic/LangOptions.h"
#include <memory>

// NOTE: these are global since cxx emits enum asserts without qualified names
using Modes = ::swift::ClangImporterOptions::Modes;

namespace cxx {
namespace swift {
namespace ClangImporterOptions {

std::unique_ptr<::swift::ClangImporterOptions> make();

std::string const &clangPath(::swift::ClangImporterOptions const &This);

void set_clangPath(::swift::ClangImporterOptions &This, rust::Str value);

std::string const &ModuleCachePath(::swift::ClangImporterOptions const &This);

void set_ModuleCachePath(::swift::ClangImporterOptions &This, rust::Str value);

std::vector<std::string> const &
ExtraArgs(::swift::ClangImporterOptions const &This);

void set_ExtraArgs(::swift::ClangImporterOptions &This,
                   rust::Slice<rust::Str const> that);

std::string const &
OverrideResourceDir(::swift::ClangImporterOptions const &This);

void set_OverrideResourceDir(::swift::ClangImporterOptions &This,
                             rust::Str value);

std::string const &TargetCPU(::swift::ClangImporterOptions const &This);

void set_TargetCPU(::swift::ClangImporterOptions &This, rust::Str value);

std::string const &IndexStorePath(::swift::ClangImporterOptions const &This);

void set_IndexStorePath(::swift::ClangImporterOptions &This, rust::Str value);

std::string const &BridgingHeader(::swift::ClangImporterOptions const &This);

void set_BridgingHeader(::swift::ClangImporterOptions &This, rust::Str value);

std::string const &
PrecompiledHeaderOutputDir(::swift::ClangImporterOptions const &This);

void set_PrecompiledHeaderOutputDir(::swift::ClangImporterOptions &This,
                                    rust::Str value);

std::string const &Optimization(::swift::ClangImporterOptions const &This);

void set_Optimization(::swift::ClangImporterOptions &This, rust::Str value);

bool PCHDisableValidation(::swift::ClangImporterOptions const &This);

void set_PCHDisableValidation(::swift::ClangImporterOptions &This, bool value);

Modes Mode(::swift::ClangImporterOptions const &This);

void set_Mode(::swift::ClangImporterOptions &This, Modes value);

bool DetailedPreprocessingRecord(::swift::ClangImporterOptions const &This);

void set_DetailedPreprocessingRecord(::swift::ClangImporterOptions &This,
                                     bool value);

bool DumpClangDiagnostics(::swift::ClangImporterOptions const &This);

void set_DumpClangDiagnostics(::swift::ClangImporterOptions &This, bool value);

bool ImportForwardDeclarations(::swift::ClangImporterOptions const &This);

void set_ImportForwardDeclarations(::swift::ClangImporterOptions &This,
                                   bool value);

bool DisableSwiftBridgeAttr(::swift::ClangImporterOptions const &This);

void set_DisableSwiftBridgeAttr(::swift::ClangImporterOptions &This,
                                bool value);

bool DisableOverlayModules(::swift::ClangImporterOptions const &This);

void set_DisableOverlayModules(::swift::ClangImporterOptions &This, bool value);

bool EnableClangSPI(::swift::ClangImporterOptions const &This);

void set_EnableClangSPI(::swift::ClangImporterOptions &This, bool value);

bool DebuggerSupport(::swift::ClangImporterOptions const &This);

void set_DebuggerSupport(::swift::ClangImporterOptions &This, bool value);

bool DisableSourceImport(::swift::ClangImporterOptions const &This);

void set_DisableSourceImport(::swift::ClangImporterOptions &This, bool value);

bool ExtraArgsOnly(::swift::ClangImporterOptions const &This);

void set_ExtraArgsOnly(::swift::ClangImporterOptions &This, bool value);

} // namespace ClangImporterOptions
} // namespace swift
} // namespace cxx
