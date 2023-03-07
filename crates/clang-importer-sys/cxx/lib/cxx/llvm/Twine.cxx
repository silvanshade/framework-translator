#include "cxx/llvm/Twine.hxx"

namespace cxx {
namespace llvm {
namespace Twine {

std::shared_ptr<::llvm::Twine> make() {
  return std::make_shared<::llvm::Twine>(::llvm::Twine::createNull());
}

std::shared_ptr<::llvm::Twine> from_cxx_string(const std::string &Str) {
  return std::make_shared<::llvm::Twine>(::llvm::Twine(Str));
}

std::shared_ptr<::llvm::Twine>
from_string_ref(const rust::llvm::StringRef &Str) {
  return std::make_shared<::llvm::Twine>(::llvm::Twine(*Str.ptr));
}

} // namespace Twine
} // namespace llvm
} // namespace cxx
