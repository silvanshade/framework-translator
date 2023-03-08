#pragma once

#include "ffi/swift/ast_context.rs.h"
#include "swift/ClangImporter/ClangImporter.h"
#include <memory>

namespace cxx {
namespace swift {
namespace ClangImporter {

std::unique_ptr<::swift::ClangImporter> create(rust::swift::ASTContext &ctx);

bool canReadPCH(::swift::ClangImporter &This,
                rust::llvm::StringRef const &PCHFilename);

bool emitBridgingPCH(::swift::ClangImporter &This,
                     rust::llvm::StringRef const &headerPath,
                     rust::llvm::StringRef const &outputPCHPath);

} // namespace ClangImporter
} // namespace swift
} // namespace cxx
