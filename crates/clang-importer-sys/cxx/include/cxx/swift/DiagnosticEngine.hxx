#pragma once

#include "ffi/swift/source_manager.rs.h"
#include "rust/cxx.h"
#include "swift/AST/DiagnosticEngine.h"
#include <memory>

namespace cxx {
namespace swift {
namespace DiagnosticEngine {

__attribute__((
    always_inline)) static inline std::unique_ptr<::swift::DiagnosticEngine>
make(rust::swift::SourceManager &SourceMgr) {
  ::swift::SourceManager &mgr = *SourceMgr.ptr;
  ::swift::DiagnosticEngine *diag = new ::swift::DiagnosticEngine(mgr);
  return std::unique_ptr<::swift::DiagnosticEngine>(diag);
}

} // namespace DiagnosticEngine
} // namespace swift
} // namespace cxx
