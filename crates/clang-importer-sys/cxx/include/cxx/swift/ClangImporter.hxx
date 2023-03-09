#pragma once

#include "ffi/swift/ast_context.rs.h"
#include "swift/ClangImporter/ClangImporter.h"
#include <memory>

namespace cxx {
namespace swift {
namespace ClangImporter {

__attribute__((
    always_inline)) static inline std::unique_ptr<::swift::ClangImporter>
create(rust::swift::ASTContext &ctx) {
  return ::swift::ClangImporter::create(*ctx.ptr);
}

__attribute__((always_inline)) static inline bool
canReadPCH(::swift::ClangImporter &This,
           rust::llvm::StringRef const &PCHFilename) {
  return This.canReadPCH(*PCHFilename.ptr);
}

__attribute__((always_inline)) static inline bool
emitBridgingPCH(::swift::ClangImporter &This,
                rust::llvm::StringRef const &headerPath,
                rust::llvm::StringRef const &outputPCHPath) {
  return This.emitBridgingPCH(*headerPath.ptr, *outputPCHPath.ptr);
}

} // namespace ClangImporter
} // namespace swift
} // namespace cxx
