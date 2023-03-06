#include "cxx/swift/ClangImporter.hxx"

namespace cxx {
namespace swift {
namespace ClangImporter {

std::unique_ptr<::swift::ClangImporter> create(rust::swift::ASTContext &ctx) {
  return ::swift::ClangImporter::create(*ctx.ptr);
}

} // namespace ClangImporter
} // namespace swift
} // namespace cxx
