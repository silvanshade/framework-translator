#pragma once

#include "swift/SymbolGraphGen/SymbolGraphOptions.h"
#include <memory>

namespace cxx {
namespace swift {
namespace symbolgraphgen {
namespace SymbolGraphOptions {

std::unique_ptr<::swift::symbolgraphgen::SymbolGraphOptions> make();

}
} // namespace symbolgraphgen
} // namespace swift
} // namespace cxx
