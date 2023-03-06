#pragma once

#include "swift/Basic/LangOptions.h"
#include <memory>

namespace cxx {
namespace swift {
namespace ClangImporterOptions {

std::unique_ptr<::swift::ClangImporterOptions> make();

}
} // namespace swift
} // namespace cxx
