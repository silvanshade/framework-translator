#pragma once

#include "ffi/llvm/string_ref.rs.h"
#include "rust/cxx.h"
#include "swift/AST/ASTContext.h"
#include "swift/Basic/LangOptions.h"

#include <memory>

namespace cxx {
namespace swift {
namespace ASTContext {

[[gnu::always_inline]] static inline std::unique_ptr<::swift::ASTContext>
get(
  ::swift::LangOptions& langOpts,
  ::swift::TypeCheckerOptions& typeckOpts,
  ::swift::SILOptions& silOpts,
  ::swift::SearchPathOptions& SearchPathOpts,
  ::swift::ClangImporterOptions& ClangImporterOpts,
  ::swift::symbolgraphgen::SymbolGraphOptions& SymbolGraphOpts,
  ::swift::SourceManager& SourceMgr,
  ::swift::DiagnosticEngine& Diags)
{
  auto raw = ::swift::ASTContext::get(
    langOpts, typeckOpts, silOpts, SearchPathOpts, ClangImporterOpts, SymbolGraphOpts, SourceMgr, Diags);
  return std::unique_ptr<::swift::ASTContext>(raw);
}

[[gnu::always_inline]] static inline std::unique_ptr<::swift::ASTContext>
getWithCallback(
  ::swift::LangOptions& langOpts,
  ::swift::TypeCheckerOptions& typeckOpts,
  ::swift::SILOptions& silOpts,
  ::swift::SearchPathOptions& SearchPathOpts,
  ::swift::ClangImporterOptions& ClangImporterOpts,
  ::swift::symbolgraphgen::SymbolGraphOptions& SymbolGraphOpts,
  ::swift::SourceManager& SourceMgr,
  ::swift::DiagnosticEngine& Diags,
  rust::Fn<bool(rust::Str, bool)> PreModuleImportCallback)
{
  std::function<bool(::llvm::StringRef, bool)> lambda = [=](::llvm::StringRef s, bool b) {
    return (*PreModuleImportCallback)(rust::Str(s.data()), b);
  };
  auto raw = ::swift::ASTContext::get(
    langOpts, typeckOpts, silOpts, SearchPathOpts, ClangImporterOpts, SymbolGraphOpts, SourceMgr, Diags, lambda);
  return std::unique_ptr<::swift::ASTContext>(raw);
}

} // namespace ASTContext
} // namespace swift
} // namespace cxx
