#include "cxx/swift/TypeCheckerOptions.hxx"

namespace cxx {
namespace swift {
namespace TypeCheckerOptions {

std::unique_ptr<::swift::TypeCheckerOptions> make() {
  return std::make_unique<::swift::TypeCheckerOptions>(
      ::swift::TypeCheckerOptions());
}

} // namespace TypeCheckerOptions
} // namespace swift
} // namespace cxx
