#pragma once

#include "llvm/ADT/StringRef.h"
#include <memory>

namespace cxx {
namespace llvm {
namespace StringRef {

std::shared_ptr<::llvm::StringRef> make();

std::shared_ptr<::llvm::StringRef> from_cxx_string(std::string const &Str);

bool equals(::llvm::StringRef const &LHS, ::llvm::StringRef const &RHS);

bool equals_insensitive(::llvm::StringRef const &LHS,
                        ::llvm::StringRef const &RHS);

} // namespace StringRef
} // namespace llvm
} // namespace cxx
