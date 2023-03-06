#include "cxx/swift/SourceManager.hxx"

namespace cxx {
namespace swift {
namespace SourceManager {

std::unique_ptr<::swift::SourceManager> make() {
  return std::make_unique<::swift::SourceManager>(::swift::SourceManager());
}

} // namespace SourceManager
} // namespace swift
} // namespace cxx
