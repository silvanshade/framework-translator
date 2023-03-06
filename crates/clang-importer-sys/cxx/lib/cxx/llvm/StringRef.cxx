#include "cxx/llvm/StringRef.hxx"

namespace cxx {
namespace llvm {
namespace StringRef {

std::unique_ptr<::llvm::StringRef> make() {
  return std::make_unique<::llvm::StringRef>(::llvm::StringRef());
}

std::unique_ptr<::llvm::StringRef> from_cxx_string(const std::string &Str) {
  return std::make_unique<::llvm::StringRef>(::llvm::StringRef(Str));
}

bool equals(const ::llvm::StringRef &LHS, const ::llvm::StringRef &RHS) {
  return LHS.equals(RHS);
}

bool equals_insensitive(const ::llvm::StringRef &LHS,
                        const ::llvm::StringRef &RHS) {
  return LHS.equals_insensitive(RHS);
}

} // namespace StringRef
} // namespace llvm
} // namespace cxx
