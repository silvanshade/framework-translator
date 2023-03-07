#pragma once

#include "ffi/llvm/string_ref.rs.h"
#include "llvm/ADT/Twine.h"
#include <memory>

namespace cxx {
namespace llvm {
namespace Twine {

std::shared_ptr<::llvm::Twine> make();

std::shared_ptr<::llvm::Twine> from_cxx_string(const std::string &Str);

std::shared_ptr<::llvm::Twine>
from_string_ref(const rust::llvm::StringRef &Str);

} // namespace Twine
} // namespace llvm
} // namespace cxx
