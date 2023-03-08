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

std::unique_ptr<::llvm::Triple> make();

std::unique_ptr<::llvm::Triple> from_twine(const rust::llvm::Twine &Str);

std::unique_ptr<::llvm::Triple>
from_arch_vendor_os(const rust::llvm::Twine &ArchStr,
                    const rust::llvm::Twine &VendorStr,
                    const rust::llvm::Twine &OSStr);

} // namespace Triple
} // namespace llvm
} // namespace cxx
