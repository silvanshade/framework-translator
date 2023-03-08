#include "cxx/llvm/StringRef.hxx"

namespace cxx {
namespace llvm {
namespace StringRef {

std::shared_ptr<::llvm::StringRef> make() {
  return std::make_shared<::llvm::StringRef>(::llvm::StringRef());
}

std::shared_ptr<::llvm::StringRef> from_cxx_string(std::string const &Str) {
  return std::make_shared<::llvm::StringRef>(::llvm::StringRef(Str));
}

bool equals(::llvm::StringRef const &LHS, ::llvm::StringRef const &RHS) {
  return LHS.equals(RHS);
}

bool equals_insensitive(::llvm::StringRef const &LHS,
                        ::llvm::StringRef const &RHS) {
  return LHS.equals_insensitive(RHS);
}

} // namespace StringRef
} // namespace llvm
} // namespace cxx
