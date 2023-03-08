#pragma once

#include "ffi/llvm/string_ref.rs.h"
#include "llvm/ADT/Twine.h"
#include <memory>

namespace cxx {
namespace llvm {
namespace Twine {

std::shared_ptr<::llvm::Twine> make();

std::shared_ptr<::llvm::Twine> from_cxx_string(std::string const &Str);

std::shared_ptr<::llvm::Twine>
from_string_ref(rust::llvm::StringRef const &Str);

} // namespace Twine
} // namespace llvm
} // namespace cxx
