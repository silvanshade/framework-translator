#pragma once

#include "ffi/llvm/twine.rs.h"
#include "llvm/ADT/Triple.h"
#include <memory>

// NOTE: these are global since cxx emits enum asserts without qualified names
using ArchType = ::llvm::Triple::ArchType;
using SubArchType = ::llvm::Triple::SubArchType;
using VendorType = ::llvm::Triple::VendorType;
using OSType = ::llvm::Triple::OSType;
using EnvironmentType = ::llvm::Triple::EnvironmentType;
using ObjectFormatType = ::llvm::Triple::ObjectFormatType;

namespace cxx {
namespace llvm {
namespace Triple {

__attribute__((always_inline)) static inline std::unique_ptr<::llvm::Triple>
make() {
  return std::make_unique<::llvm::Triple>(::llvm::Triple());
}

__attribute__((always_inline)) static inline std::unique_ptr<::llvm::Triple>
from_twine(rust::llvm::Twine const &Str) {
  return std::make_unique<::llvm::Triple>(::llvm::Triple(*Str.ptr));
}

__attribute__((always_inline)) static inline std::unique_ptr<::llvm::Triple>
from_arch_vendor_os(rust::llvm::Twine const &ArchStr,
                    rust::llvm::Twine const &VendorStr,
                    rust::llvm::Twine const &OSStr) {
  return std::make_unique<::llvm::Triple>(
      ::llvm::Triple(*ArchStr.ptr, *VendorStr.ptr, *OSStr.ptr));
}

} // namespace Triple
} // namespace llvm
} // namespace cxx
