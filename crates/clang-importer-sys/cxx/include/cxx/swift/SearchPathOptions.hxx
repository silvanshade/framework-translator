#pragma once

#include "swift/AST/SearchPathOptions.h"
#include <memory>

namespace cxx {
namespace swift {
namespace SearchPathOptions {

__attribute__((
    always_inline)) static inline std::unique_ptr<::swift::SearchPathOptions>
make() {
  return std::make_unique<::swift::SearchPathOptions>(
      ::swift::SearchPathOptions());
}

} // namespace SearchPathOptions
} // namespace swift
} // namespace cxx
