#pragma once

#include "swift/Basic/LangOptions.h"
#include "llvm/ADT/Triple.h"
#include <memory>

namespace cxx {
namespace swift {
namespace LangOptions {

__attribute__((
    always_inline)) static inline std::unique_ptr<::swift::LangOptions>
make() {
  return std::make_unique<::swift::LangOptions>(::swift::LangOptions());
}

__attribute__((always_inline)) static inline std::unique_ptr<::llvm::Triple>
Target(::swift::LangOptions const &This) {
  return std::make_unique<::llvm::Triple>(This.Target);
}

__attribute__((always_inline)) static inline void
SetTarget(::swift::LangOptions &This, std::unique_ptr<::llvm::Triple> target) {
  This.Target = *target;
}

} // namespace LangOptions
} // namespace swift
} // namespace cxx
