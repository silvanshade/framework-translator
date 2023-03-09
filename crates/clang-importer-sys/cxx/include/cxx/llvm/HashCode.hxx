#pragma once

#include "llvm/ADT/Hashing.h"

#include <memory>

namespace cxx {
namespace llvm {
namespace HashCode {

[[gnu::always_inline]] static inline std::unique_ptr<::llvm::hash_code>
make()
{
  return std::make_unique<::llvm::hash_code>(::llvm::hash_code());
}

[[gnu::always_inline]] static inline std::size_t
value(::llvm::hash_code const& This)
{
  return size_t(This);
}

}
}
}
