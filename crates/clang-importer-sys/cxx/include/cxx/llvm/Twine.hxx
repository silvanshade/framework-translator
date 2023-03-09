#pragma once

#include "ffi/llvm/string_ref.rs.h"
#include "llvm/ADT/Twine.h"
#include <memory>

namespace cxx {
namespace llvm {
namespace Twine {

__attribute__((always_inline)) static inline std::shared_ptr<::llvm::Twine>
make() {
  return std::make_shared<::llvm::Twine>(::llvm::Twine::createNull());
}

__attribute__((always_inline)) static inline std::shared_ptr<::llvm::Twine>
from_cxx_string(std::string const &Str) {
  return std::make_shared<::llvm::Twine>(::llvm::Twine(Str));
}

__attribute__((always_inline)) static inline std::shared_ptr<::llvm::Twine>
from_string_ref(rust::llvm::StringRef const &Str) {
  return std::make_shared<::llvm::Twine>(::llvm::Twine(*Str.ptr));
}

} // namespace Twine
} // namespace llvm
} // namespace cxx
