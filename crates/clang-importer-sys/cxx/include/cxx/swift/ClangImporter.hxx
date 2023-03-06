#pragma once

#include "ffi/swift/ast_context.rs.h"
#include "swift/ClangImporter/ClangImporter.h"
#include <memory>

namespace cxx {
namespace swift {
namespace ClangImporter {

std::unique_ptr<::swift::ClangImporter> create(rust::swift::ASTContext &ctx);

}
} // namespace swift
} // namespace cxx
