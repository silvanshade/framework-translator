#pragma once

#include "llvm/ADT/StringRef.h"

#include <memory>

namespace cxx {
namespace llvm {
namespace StringRef {

[[gnu::always_inline]] static inline std::shared_ptr<::llvm::StringRef>
make()
{
  return std::make_shared<::llvm::StringRef>(::llvm::StringRef());
}

[[gnu::always_inline]] static inline std::shared_ptr<::llvm::StringRef>
from_cxx_string(std::string const& Str)
{
  return std::make_shared<::llvm::StringRef>(::llvm::StringRef(Str));
}

[[gnu::always_inline]] static inline bool
equals(::llvm::StringRef const& LHS, ::llvm::StringRef const& RHS)
{
  return LHS.equals(RHS);
}

[[gnu::always_inline]] static inline bool
equals_insensitive(::llvm::StringRef const& LHS, ::llvm::StringRef const& RHS)
{
  return LHS.equals_insensitive(RHS);
}

} // namespace StringRef
} // namespace llvm
} // namespace cxx
