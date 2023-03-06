#pragma once

#include "ffi/swift/source_manager.rs.h"
#include "rust/cxx.h"
#include "swift/AST/DiagnosticEngine.h"
#include <memory>

namespace cxx {
namespace swift {
namespace DiagnosticEngine {

std::unique_ptr<::swift::DiagnosticEngine>
make(rust::swift::SourceManager &SourceMgr);

}
} // namespace swift
} // namespace cxx
