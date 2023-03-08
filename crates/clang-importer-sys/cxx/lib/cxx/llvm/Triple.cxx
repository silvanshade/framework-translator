#include "cxx/llvm/Triple.hxx"

namespace cxx {
namespace llvm {
namespace Triple {

std::unique_ptr<::llvm::Triple> make() {
  return std::make_unique<::llvm::Triple>(::llvm::Triple());
}

std::unique_ptr<::llvm::Triple> from_twine(rust::llvm::Twine const &Str) {
  return std::make_unique<::llvm::Triple>(::llvm::Triple(*Str.ptr));
}

std::unique_ptr<::llvm::Triple>
from_arch_vendor_os(rust::llvm::Twine const &ArchStr,
                    rust::llvm::Twine const &VendorStr,
                    rust::llvm::Twine const &OSStr) {
  return std::make_unique<::llvm::Triple>(
      ::llvm::Triple(*ArchStr.ptr, *VendorStr.ptr, *OSStr.ptr));
}

} // namespace Triple
} // namespace llvm
} // namespace cxx