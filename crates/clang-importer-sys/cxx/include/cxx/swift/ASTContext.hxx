#pragma once

#include "ffi/llvm/string_ref.rs.h"
#include "ffi/swift/ast_context.rs.h"
#include "ffi/swift/clang_importer.rs.h"
#include "ffi/swift/clang_importer_options.rs.h"
#include "ffi/swift/diagnostic_engine.rs.h"
#include "ffi/swift/lang_options.rs.h"
#include "ffi/swift/search_path_options.rs.h"
#include "ffi/swift/sil_options.rs.h"
#include "ffi/swift/source_manager.rs.h"
#include "ffi/swift/symbolgraphgen/symbol_graph_options.rs.h"
#include "ffi/swift/type_checker_options.rs.h"
#include <memory>

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
    rust::swift::DiagnosticEngine &Diags);

std::unique_ptr<::swift::ASTContext> getWithCallback(
    rust::swift::LangOptions &langOpts,
    rust::swift::TypeCheckerOptions &typeckOpts,
    rust::swift::SILOptions &silOpts,
    rust::swift::SearchPathOptions &SearchPathOpts,
    rust::swift::ClangImporterOptions &ClangImporterOpts,
    rust::swift::symbolgraphgen::SymbolGraphOptions &SymbolGraphOpts,
    rust::swift::SourceManager &SourceMgr, rust::swift::DiagnosticEngine &Diags,
    rust::Fn<bool(rust::llvm::StringRef, bool)> PreModuleImportCallback);

} // namespace ASTContext
} // namespace swift
} // namespace cxx
