#pragma once

#include "llvm/ADT/Triple.h"
#include "llvm/ADT/Twine.h"

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

[[gnu::always_inline]] static inline std::unique_ptr<::llvm::Triple>
make()
{
  return std::make_unique<::llvm::Triple>(::llvm::Triple());
}

[[gnu::always_inline]] static inline std::unique_ptr<::llvm::Triple>
from_twine(::llvm::Twine const& Str)
{
  return std::make_unique<::llvm::Triple>(::llvm::Triple(Str));
}

[[gnu::always_inline]] static inline std::unique_ptr<::llvm::Triple>
from_arch_vendor_os(::llvm::Twine const& ArchStr, ::llvm::Twine const& VendorStr, ::llvm::Twine const& OSStr)
{
  return std::make_unique<::llvm::Triple>(::llvm::Triple(ArchStr, VendorStr, OSStr));
}

} // namespace Triple
} // namespace llvm
} // namespace cxx
