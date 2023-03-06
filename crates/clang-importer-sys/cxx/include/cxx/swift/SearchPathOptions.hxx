#pragma once

#include "swift/AST/SearchPathOptions.h"
#include <memory>

namespace cxx {
namespace swift {
namespace SearchPathOptions {

std::unique_ptr<::swift::SearchPathOptions> make();

}
} // namespace swift
} // namespace cxx
