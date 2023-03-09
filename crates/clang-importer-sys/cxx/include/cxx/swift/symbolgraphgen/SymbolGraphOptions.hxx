#pragma once

#include "swift/SymbolGraphGen/SymbolGraphOptions.h"

#include <memory>

namespace cxx {
namespace swift {
namespace symbolgraphgen {
namespace SymbolGraphOptions {

[[gnu::always_inline]] static inline std::unique_ptr<::swift::symbolgraphgen::SymbolGraphOptions>
make()
{
  return std::make_unique<::swift::symbolgraphgen::SymbolGraphOptions>(::swift::symbolgraphgen::SymbolGraphOptions());
}

} // namespace SymbolGraphOptions
} // namespace symbolgraphgen
} // namespace swift
} // namespace cxx
