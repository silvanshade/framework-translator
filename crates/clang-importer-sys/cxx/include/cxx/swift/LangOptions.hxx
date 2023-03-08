#pragma once

#include "swift/Basic/LangOptions.h"
#include "llvm/ADT/Triple.h"
#include <memory>

namespace cxx {
namespace swift {
namespace LangOptions {

std::unique_ptr<::swift::LangOptions> make();

std::unique_ptr<::llvm::Triple> Target(::swift::LangOptions const &This);

void SetTarget(::swift::LangOptions &This,
               std::unique_ptr<::llvm::Triple> target);

} // namespace LangOptions
} // namespace swift
} // namespace cxx
