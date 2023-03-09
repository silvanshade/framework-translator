#pragma once

#include "rust/cxx.h"
#include "swift/AST/DiagnosticEngine.h"
#include "swift/Basic/SourceManager.h"

#include <memory>

namespace cxx {
namespace swift {
namespace DiagnosticEngine {

[[gnu::always_inline]] static inline std::unique_ptr<::swift::DiagnosticEngine>
make(::swift::SourceManager& mgr)
{
  ::swift::DiagnosticEngine* diag = new ::swift::DiagnosticEngine(mgr);
  return std::unique_ptr<::swift::DiagnosticEngine>(diag);
}

} // namespace DiagnosticEngine
} // namespace swift
} // namespace cxx
