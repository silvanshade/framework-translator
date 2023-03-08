#include "cxx/swift/ClangImporter.hxx"

namespace cxx {
namespace swift {
namespace ClangImporter {

std::unique_ptr<::swift::ClangImporter> create(rust::swift::ASTContext &ctx) {
  return ::swift::ClangImporter::create(*ctx.ptr);
}

bool canReadPCH(::swift::ClangImporter &This,
                rust::llvm::StringRef const &PCHFilename) {
  return This.canReadPCH(*PCHFilename.ptr);
}

bool emitBridgingPCH(::swift::ClangImporter &This,
                     rust::llvm::StringRef const &headerPath,
                     rust::llvm::StringRef const &outputPCHPath) {
  return This.emitBridgingPCH(*headerPath.ptr, *outputPCHPath.ptr);
}

} // namespace ClangImporter
} // namespace swift
} // namespace cxx
