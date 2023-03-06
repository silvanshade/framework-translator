#include "cxx/swift/SILOptions.hxx"

namespace cxx {
namespace swift {
namespace SILOptions {

std::unique_ptr<::swift::SILOptions> make() {
  return std::make_unique<::swift::SILOptions>(::swift::SILOptions());
}

} // namespace SILOptions
} // namespace swift
} // namespace cxx
