#pragma once

#include "rust/cxx.h"
#include "swift/Basic/LangOptions.h"

#include "llvm/ADT/SmallVector.h"

#include <memory>

namespace cxx {
namespace swift {
namespace TypeCheckerOptions {

[[gnu::always_inline]] static inline std::unique_ptr<::swift::TypeCheckerOptions>
make()
{
  return std::make_unique<::swift::TypeCheckerOptions>(::swift::TypeCheckerOptions());
}

[[gnu::always_inline]] static inline std::uint32_t
WarnLongFunctionBodies(::swift::TypeCheckerOptions const& This)
{
  return This.WarnLongFunctionBodies;
}

[[gnu::always_inline]] void
set_WarnLongFunctionBodies(::swift::TypeCheckerOptions& This, std::uint32_t value)
{
  This.WarnLongFunctionBodies = value;
}

[[gnu::always_inline]] static inline std::uint32_t
WarnLongExpressionTypeChecking(::swift::TypeCheckerOptions const& This)
{
  return This.WarnLongExpressionTypeChecking;
}

[[gnu::always_inline]] void
set_WarnLongExpressionTypeChecking(::swift::TypeCheckerOptions& This, std::uint32_t value)
{
  This.WarnLongExpressionTypeChecking = value;
}

[[gnu::always_inline]] static inline std::uint32_t
ExpressionTimeoutThreshold(::swift::TypeCheckerOptions const& This)
{
  return This.ExpressionTimeoutThreshold;
}

[[gnu::always_inline]] static inline void
set_ExpressionTimeoutThreshold(::swift::TypeCheckerOptions& This, std::uint32_t value)
{
  This.ExpressionTimeoutThreshold = value;
}

[[gnu::always_inline]] static inline std::uint32_t
SwitchCheckingInvocationThreshold(::swift::TypeCheckerOptions const& This)
{
  return This.SwitchCheckingInvocationThreshold;
}

[[gnu::always_inline]] static inline void
set_SwitchCheckingInvocationThreshold(::swift::TypeCheckerOptions& This, std::uint32_t value)
{
  This.SwitchCheckingInvocationThreshold = value;
}

[[gnu::always_inline]] static inline bool
DebugTimeFunctionBodies(::swift::TypeCheckerOptions const& This)
{
  return This.DebugTimeFunctionBodies;
}

[[gnu::always_inline]] static inline void
set_DebugTimeFunctionBodies(::swift::TypeCheckerOptions& This, bool value)
{
  This.DebugTimeFunctionBodies = value;
}

[[gnu::always_inline]] static inline bool
DebugTimeExpressions(::swift::TypeCheckerOptions const& This)
{
  return This.DebugTimeExpressions;
}

[[gnu::always_inline]] static inline void
set_DebugTimeExpressions(::swift::TypeCheckerOptions& This, bool value)
{
  This.DebugTimeExpressions = value;
}

[[gnu::always_inline]] static inline ::swift::FunctionBodySkipping
SkipFunctionBodies(::swift::TypeCheckerOptions const& This)
{
  return This.SkipFunctionBodies;
}

[[gnu::always_inline]] static inline void
set_SkipFunctionBodies(::swift::TypeCheckerOptions& This, ::swift::FunctionBodySkipping value)
{
  This.SkipFunctionBodies = value;
}

[[gnu::always_inline]] static inline bool
DebugGenericSignatures(::swift::TypeCheckerOptions const& This)
{
  return This.DebugGenericSignatures;
}

[[gnu::always_inline]] static inline void
set_DebugGenericSignatures(::swift::TypeCheckerOptions& This, bool value)
{
  This.DebugGenericSignatures = value;
}

[[gnu::always_inline]] static inline bool
DebugConstraintSolver(::swift::TypeCheckerOptions const& This)
{
  return This.DebugConstraintSolver;
}

[[gnu::always_inline]] static inline void
set_DebugConstraintSolver(::swift::TypeCheckerOptions& This, bool value)
{
  This.DebugConstraintSolver = value;
}

[[gnu::always_inline]] static inline bool
DebugConstraintSolverAttempt(::swift::TypeCheckerOptions const& This)
{
  return This.DebugConstraintSolverAttempt;
}

[[gnu::always_inline]] static inline void
set_DebugConstraintSolverAttempt(::swift::TypeCheckerOptions& This, bool value)
{
  This.DebugConstraintSolverAttempt = value;
}

[[gnu::always_inline]] static inline rust::Vec<std::uint32_t>
DebugConstraintSolverOnLines(::swift::TypeCheckerOptions const& This)
{
  auto vec = rust::Vec<std::uint32_t>();
  vec.reserve(This.DebugConstraintSolverOnLines.size());
  for (auto const elem : This.DebugConstraintSolverOnLines) {
    vec.push_back(elem);
  }
  return vec;
}

[[gnu::always_inline]] static inline void
set_DebugConstraintSolverOnLines(::swift::TypeCheckerOptions& This, rust::Slice<std::uint32_t const> that)
{
  auto thisSize = This.DebugConstraintSolverOnLines.size();
  auto thatSize = that.size();
  This.DebugConstraintSolverOnLines.reserve(thatSize);
  std::size_t i = 0;
  // overwrite initial existing
  for (; i != thatSize && i != thisSize; ++i) {
    This.DebugConstraintSolverOnLines[i] = that[i];
  }
  if (thatSize < thisSize) {
    // erase trailing
    This.DebugConstraintSolverOnLines.erase(
      This.DebugConstraintSolverOnLines.begin() + thatSize, This.DebugConstraintSolverOnLines.end());
  } else {
    // subsequently extend with additional
    for (; i != thatSize; ++i) {
      This.DebugConstraintSolverOnLines.emplace_back(that[i]);
    }
  }
}

[[gnu::always_inline]] static inline std::string const&
DebugForbidTypecheckPrefix(::swift::TypeCheckerOptions const& This)
{
  return This.DebugForbidTypecheckPrefix;
}

[[gnu::always_inline]] static inline void
set_DebugForbidTypecheckPrefix(::swift::TypeCheckerOptions& This, rust::Str value)
{
  This.DebugForbidTypecheckPrefix = std::string(value);
}

[[gnu::always_inline]] static inline std::uint32_t
SolverMemoryThreshold(::swift::TypeCheckerOptions const& This)
{
  return This.SolverMemoryThreshold;
}

[[gnu::always_inline]] static inline void
set_SolverMemoryThreshold(::swift::TypeCheckerOptions& This, std::uint32_t value)
{
  This.SolverMemoryThreshold = value;
}

[[gnu::always_inline]] static inline std::uint32_t
SolverBindingThreshold(::swift::TypeCheckerOptions const& This)
{
  return This.SolverBindingThreshold;
}

[[gnu::always_inline]] static inline void
set_SolverBindingThreshold(::swift::TypeCheckerOptions& This, std::uint32_t value)
{
  This.SolverBindingThreshold = value;
}

[[gnu::always_inline]] static inline std::uint32_t
SolverShrinkUnsolvedThreshold(::swift::TypeCheckerOptions const& This)
{
  return This.SolverShrinkUnsolvedThreshold;
}

[[gnu::always_inline]] static inline void
set_SolverShrinkUnsolvedThreshold(::swift::TypeCheckerOptions& This, std::uint32_t value)
{
  This.SolverShrinkUnsolvedThreshold = value;
}

[[gnu::always_inline]] static inline bool
SolverDisableShrink(::swift::TypeCheckerOptions const& This)
{
  return This.SolverDisableShrink;
}

[[gnu::always_inline]] static inline void
set_SolverDisableShrink(::swift::TypeCheckerOptions& This, bool value)
{
  This.SolverDisableShrink = value;
}

[[gnu::always_inline]] static inline bool
EnableOperatorDesignatedTypes(::swift::TypeCheckerOptions const& This)
{
  return This.EnableOperatorDesignatedTypes;
}

[[gnu::always_inline]] static inline void
set_EnableOperatorDesignatedTypes(::swift::TypeCheckerOptions& This, bool value)
{
  This.EnableOperatorDesignatedTypes = value;
}

[[gnu::always_inline]] static inline bool
DisableConstraintSolverPerformanceHacks(::swift::TypeCheckerOptions const& This)
{
  return This.DisableConstraintSolverPerformanceHacks;
}

[[gnu::always_inline]] static inline void
set_DisableConstraintSolverPerformanceHacks(::swift::TypeCheckerOptions& This, bool value)
{
  This.DisableConstraintSolverPerformanceHacks = value;
}

[[gnu::always_inline]] static inline bool
EnableOneWayClosureParameters(::swift::TypeCheckerOptions const& This)
{
  return This.EnableOneWayClosureParameters;
}

[[gnu::always_inline]] static inline void
set_EnableOneWayClosureParameters(::swift::TypeCheckerOptions& This, bool value)
{
  This.EnableOneWayClosureParameters = value;
}

[[gnu::always_inline]] static inline bool
EnableMultiStatementClosureInference(::swift::TypeCheckerOptions const& This)
{
  return This.EnableMultiStatementClosureInference;
}

[[gnu::always_inline]] static inline void
set_EnableMultiStatementClosureInference(::swift::TypeCheckerOptions& This, bool value)
{
  This.EnableMultiStatementClosureInference = value;
}

[[gnu::always_inline]] static inline bool
PrintFullConvention(::swift::TypeCheckerOptions const& This)
{
  return This.PrintFullConvention;
}

[[gnu::always_inline]] static inline void
set_PrintFullConvention(::swift::TypeCheckerOptions& This, bool value)
{
  This.PrintFullConvention = value;
}

} // namespace TypeCheckerOptions
} // namespace swift
} // namespace cxx
