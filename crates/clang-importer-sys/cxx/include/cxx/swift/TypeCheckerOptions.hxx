#pragma once

#include "swift/Basic/LangOptions.h"
#include <memory>

namespace cxx {
namespace swift {
namespace TypeCheckerOptions {

std::unique_ptr<::swift::TypeCheckerOptions> make();

}
} // namespace swift
} // namespace cxx
