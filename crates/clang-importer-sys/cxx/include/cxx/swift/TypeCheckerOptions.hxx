#pragma once

#include "swift/Basic/LangOptions.h"
#include <memory>

namespace cxx {
namespace swift {
namespace TypeCheckerOptions {

__attribute__((
    always_inline)) static inline std::unique_ptr<::swift::TypeCheckerOptions>
make() {
  return std::make_unique<::swift::TypeCheckerOptions>(
      ::swift::TypeCheckerOptions());
}

} // namespace TypeCheckerOptions
} // namespace swift
} // namespace cxx
