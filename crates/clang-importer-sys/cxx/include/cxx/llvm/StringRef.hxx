#ifndef CXX_LLVM_STRINGREF_HXX
#define CXX_LLVM_STRINGREF_HXX

#include "llvm/ADT/StringRef.h"
#include <memory>

namespace cxx {
namespace llvm {
namespace StringRef {

std::unique_ptr<::llvm::StringRef> make();

std::unique_ptr<::llvm::StringRef> from_cxx_string(const std::string &Str);

bool equals(const ::llvm::StringRef &LHS, const ::llvm::StringRef &RHS);

bool equals_insensitive(const ::llvm::StringRef &LHS,
                        const ::llvm::StringRef &RHS);

} // namespace StringRef
} // namespace llvm
} // namespace cxx

#endif // CXX_LLVM
