#pragma once

#include "swift/Basic/FunctionBodySkipping.h"

// NOTE: these are global since cxx emits enum asserts without qualified names
using FunctionBodySkipping = ::swift::FunctionBodySkipping;
