#pragma once

#include "swift/AST/SILOptions.h"

#include <memory>

namespace cxx {
namespace swift {
namespace SILOptions {

[[gnu::always_inline]] static inline std::unique_ptr<::swift::SILOptions>
make()
{
  return std::make_unique<::swift::SILOptions>(::swift::SILOptions());
}

} // namespace SILOptions
} // namespace swift
} // namespace cxx
