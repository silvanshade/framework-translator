#include "cxx/swift/ClangImporterOptions.hxx"

namespace cxx {
namespace swift {
namespace ClangImporterOptions {

std::unique_ptr<::swift::ClangImporterOptions> make() {
  return std::make_unique<::swift::ClangImporterOptions>(
      ::swift::ClangImporterOptions());
}

} // namespace ClangImporterOptions
} // namespace swift
} // namespace cxx
