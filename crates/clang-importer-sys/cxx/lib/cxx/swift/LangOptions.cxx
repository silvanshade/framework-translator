#include "cxx/swift/LangOptions.hxx"

namespace cxx {
namespace swift {
namespace LangOptions {

std::unique_ptr<::swift::LangOptions> make() {
  return std::make_unique<::swift::LangOptions>(::swift::LangOptions());
}

std::unique_ptr<::llvm::Triple> Target(::swift::LangOptions const &This) {
  return std::make_unique<::llvm::Triple>(This.Target);
}

void SetTarget(::swift::LangOptions &This,
               std::unique_ptr<::llvm::Triple> target) {
  This.Target = *target;
}

} // namespace LangOptions
} // namespace swift
} // namespace cxx
