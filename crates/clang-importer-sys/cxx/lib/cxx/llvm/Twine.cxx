#include "cxx/llvm/Twine.hxx"

namespace cxx {
namespace llvm {
namespace Twine {

std::shared_ptr<::llvm::Twine> make() {
  return std::make_shared<::llvm::Twine>(::llvm::Twine::createNull());
}

std::shared_ptr<::llvm::Twine> from_cxx_string(std::string const &Str) {
  return std::make_shared<::llvm::Twine>(::llvm::Twine(Str));
}

std::shared_ptr<::llvm::Twine>
from_string_ref(rust::llvm::StringRef const &Str) {
  return std::make_shared<::llvm::Twine>(::llvm::Twine(*Str.ptr));
}

} // namespace Twine
} // namespace llvm
} // namespace cxx
