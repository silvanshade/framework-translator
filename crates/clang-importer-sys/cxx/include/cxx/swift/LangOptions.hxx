#pragma once

#include "swift/Basic/LangOptions.h"
#include <memory>

namespace cxx {
namespace swift {
namespace LangOptions {

std::unique_ptr<::swift::LangOptions> make();

}
} // namespace swift
} // namespace cxx
