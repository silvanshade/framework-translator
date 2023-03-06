#pragma once

#include "swift/Basic/SourceManager.h"
#include <memory>

namespace cxx {
namespace swift {
namespace SourceManager {

std::unique_ptr<::swift::SourceManager> make();

}
} // namespace swift
} // namespace cxx
