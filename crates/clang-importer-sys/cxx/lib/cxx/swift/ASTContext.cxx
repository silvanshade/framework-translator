#include "cxx/swift/ASTContext.hxx"

namespace cxx {
namespace swift {
namespace ASTContext {

std::unique_ptr<::swift::ASTContext>
get(rust::swift::LangOptions &langOpts,
    rust::swift::TypeCheckerOptions &typeckOpts,
    rust::swift::SILOptions &silOpts,
    rust::swift::SearchPathOptions &SearchPathOpts,
    rust::swift::ClangImporterOptions &ClangImporterOpts,
    rust::swift::symbolgraphgen::SymbolGraphOptions &SymbolGraphOpts,
    rust::swift::SourceManager &SourceMgr,
    rust::swift::DiagnosticEngine &Diags) {
  auto raw = ::swift::ASTContext::get(
      *langOpts.ptr, *typeckOpts.ptr, *silOpts.ptr, *SearchPathOpts.ptr,
      *ClangImporterOpts.ptr, *SymbolGraphOpts.ptr, *SourceMgr.ptr, *Diags.ptr);
  return std::unique_ptr<::swift::ASTContext>(raw);
}

std::unique_ptr<::swift::ASTContext> getWithCallback(
    rust::swift::LangOptions &langOpts,
    rust::swift::TypeCheckerOptions &typeckOpts,
    rust::swift::SILOptions &silOpts,
    rust::swift::SearchPathOptions &SearchPathOpts,
    rust::swift::ClangImporterOptions &ClangImporterOpts,
    rust::swift::symbolgraphgen::SymbolGraphOptions &SymbolGraphOpts,
    rust::swift::SourceManager &SourceMgr, rust::swift::DiagnosticEngine &Diags,
    rust::Fn<bool(rust::llvm::StringRef, bool)> PreModuleImportCallback) {
  std::function<bool(::llvm::StringRef, bool)> lambda = [=](::llvm::StringRef s,
                                                            bool b) {
    auto ptr = std::make_unique<::llvm::StringRef>(s);
    rust::llvm::StringRef t = {std::move(ptr)};
    return (*PreModuleImportCallback)(std::move(t), std::move(b));
  };
  auto raw = ::swift::ASTContext::get(
      *langOpts.ptr, *typeckOpts.ptr, *silOpts.ptr, *SearchPathOpts.ptr,
      *ClangImporterOpts.ptr, *SymbolGraphOpts.ptr, *SourceMgr.ptr, *Diags.ptr,
      lambda);
  return std::unique_ptr<::swift::ASTContext>(raw);
}

} // namespace ASTContext
} // namespace swift
} // namespace cxx
