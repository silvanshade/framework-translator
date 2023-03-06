#pragma once

#include "swift/AST/SILOptions.h"
#include <memory>

namespace cxx {
namespace swift {
namespace SILOptions {

std::unique_ptr<::swift::SILOptions> make();

}
} // namespace swift
} // namespace cxx
