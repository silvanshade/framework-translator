#include "cxx/swift/DiagnosticEngine.hxx"

namespace cxx {
namespace swift {
namespace DiagnosticEngine {

std::unique_ptr<::swift::DiagnosticEngine>
make(rust::swift::SourceManager &SourceMgr) {
  ::swift::SourceManager &mgr = *SourceMgr.ptr;
  ::swift::DiagnosticEngine *diag = new ::swift::DiagnosticEngine(mgr);
  return std::unique_ptr<::swift::DiagnosticEngine>(diag);
}

} // namespace DiagnosticEngine
} // namespace swift
} // namespace cxx