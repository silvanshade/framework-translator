#pragma once

#include "swift/Basic/SourceManager.h"
#include <memory>

namespace cxx {
namespace swift {
namespace SourceManager {

__attribute__((
    always_inline)) static inline std::unique_ptr<::swift::SourceManager>
make() {
  return std::make_unique<::swift::SourceManager>(::swift::SourceManager());
}

} // namespace SourceManager
} // namespace swift
} // namespace cxx
