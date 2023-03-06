#include "cxx/swift/SearchPathOptions.hxx"

namespace cxx {
namespace swift {
namespace SearchPathOptions {

std::unique_ptr<::swift::SearchPathOptions> make() {
  return std::make_unique<::swift::SearchPathOptions>(
      ::swift::SearchPathOptions());
}

} // namespace SearchPathOptions
} // namespace swift
} // namespace cxx
