#include "cxx/swift/symbolgraphgen/SymbolGraphOptions.hxx"

namespace cxx {
namespace swift {
namespace symbolgraphgen {
namespace SymbolGraphOptions {

std::unique_ptr<::swift::symbolgraphgen::SymbolGraphOptions> make() {
  return std::make_unique<::swift::symbolgraphgen::SymbolGraphOptions>(
      ::swift::symbolgraphgen::SymbolGraphOptions());
}

} // namespace SymbolGraphOptions
} // namespace symbolgraphgen
} // namespace swift
} // namespace cxx
