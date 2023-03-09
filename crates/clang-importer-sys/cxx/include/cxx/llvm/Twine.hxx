#pragma once

#include "rust/cxx.h"

#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/Twine.h"

#include <memory>
#include <string_view>

namespace cxx {
namespace llvm {
namespace Twine {

[[gnu::always_inline]] static inline std::shared_ptr<::llvm::Twine>
make()
{
  return std::make_shared<::llvm::Twine>(::llvm::Twine::createNull());
}

[[gnu::always_inline]] static inline std::shared_ptr<::llvm::Twine>
from_cxx_string(std::string const& Str)
{
  return std::make_shared<::llvm::Twine>(::llvm::Twine(Str));
}

[[gnu::always_inline]] static inline std::shared_ptr<::llvm::Twine>
from_string_ref(::llvm::StringRef const& Str)
{
  return std::make_shared<::llvm::Twine>(::llvm::Twine(Str));
}

[[gnu::always_inline]] static inline std::shared_ptr<::llvm::Twine>
from_rust_str(rust::Str Str)
{
  auto view = std::string_view(Str.data(), Str.length());
  return std::make_shared<::llvm::Twine>(::llvm::Twine(view));
}

[[gnu::always_inline]] static inline std::unique_ptr<std::string>
str(::llvm::Twine const& This)
{
  return std::make_unique<std::string>(This.str());
}

} // namespace Twine
} // namespace llvm
} // namespace cxx
