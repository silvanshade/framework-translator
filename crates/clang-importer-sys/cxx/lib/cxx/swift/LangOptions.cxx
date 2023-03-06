#include "cxx/swift/LangOptions.hxx"

namespace cxx {
namespace swift {
namespace LangOptions {

std::unique_ptr<::swift::LangOptions> make() {
  return std::make_unique<::swift::LangOptions>(::swift::LangOptions());
}

} // namespace LangOptions
} // namespace swift
} // namespace cxx
