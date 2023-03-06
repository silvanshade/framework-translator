#ifndef CXX_SWIFT_SYMBOLGRAPHGEN_SYMBOLGRAPHOPTIONS_HXX
#define CXX_SWIFT_SYMBOLGRAPHGEN_SYMBOLGRAPHOPTIONS_HXX

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

#endif // CXX_SWIFT_SYMBOLGRAPHGEN_SYMBOLGRAPHOPTIONS_HXX
