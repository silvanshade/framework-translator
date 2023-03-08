#include "cxx/llvm/Triple.hxx"

namespace cxx {
namespace llvm {
namespace Triple {

std::unique_ptr<::llvm::Triple> make() {
  return std::make_unique<::llvm::Triple>(::llvm::Triple());
}

std::unique_ptr<::llvm::Triple> from_twine(const rust::llvm::Twine &Str) {
  return std::make_unique<::llvm::Triple>(::llvm::Triple(*Str.ptr));
}

std::unique_ptr<::llvm::Triple>
from_arch_vendor_os(const rust::llvm::Twine &ArchStr,
                    const rust::llvm::Twine &VendorStr,
                    const rust::llvm::Twine &OSStr) {
  return std::make_unique<::llvm::Triple>(
      ::llvm::Triple(*ArchStr.ptr, *VendorStr.ptr, *OSStr.ptr));
}

} // namespace Triple
} // namespace llvm
} // namespace cxx
