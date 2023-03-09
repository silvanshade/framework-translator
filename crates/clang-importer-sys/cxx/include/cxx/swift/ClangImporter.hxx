#pragma once

#include "swift/AST/ASTContext.h"
#include "swift/ClangImporter/ClangImporter.h"

#include "llvm/ADT/StringRef.h"

#include <memory>

namespace cxx {
namespace swift {
namespace ClangImporter {

[[gnu::always_inline]] static inline std::unique_ptr<::swift::ClangImporter>
create(::swift::ASTContext& ctx)
{
  return ::swift::ClangImporter::create(ctx);
}

[[gnu::always_inline]] static inline bool
canReadPCH(::swift::ClangImporter& This, ::llvm::StringRef const& PCHFilename)
{
  return This.canReadPCH(PCHFilename);
}

[[gnu::always_inline]] static inline bool
emitBridgingPCH(
  ::swift::ClangImporter& This,
  ::llvm::StringRef const& headerPath,
  ::llvm::StringRef const& outputPCHPath)
{
  return This.emitBridgingPCH(headerPath, outputPCHPath);
}

} // namespace ClangImporter
} // namespace swift
} // namespace cxx
