#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "swift"]
    extern "C++" {
        include!("swift/Basic/LangOptions.h");

        #[cxx_name = "TypeCheckerOptions"]
        type CxxTypeCheckerOptions;

        type FunctionBodySkipping = crate::swift::FunctionBodySkipping;
    }

    #[namespace = "cxx::swift::TypeCheckerOptions"]
    extern "C++" {
        include!("cxx/swift/TypeCheckerOptions.hxx");

        unsafe fn make() -> UniquePtr<CxxTypeCheckerOptions>;

        unsafe fn WarnLongFunctionBodies(This: &CxxTypeCheckerOptions) -> u32;

        unsafe fn set_WarnLongFunctionBodies(This: Pin<&mut CxxTypeCheckerOptions>, value: u32);

        unsafe fn WarnLongExpressionTypeChecking(This: &CxxTypeCheckerOptions) -> u32;

        unsafe fn set_WarnLongExpressionTypeChecking(This: Pin<&mut CxxTypeCheckerOptions>, value: u32);

        unsafe fn ExpressionTimeoutThreshold(This: &CxxTypeCheckerOptions) -> u32;

        unsafe fn set_ExpressionTimeoutThreshold(This: Pin<&mut CxxTypeCheckerOptions>, value: u32);

        unsafe fn SwitchCheckingInvocationThreshold(This: &CxxTypeCheckerOptions) -> u32;

        unsafe fn set_SwitchCheckingInvocationThreshold(This: Pin<&mut CxxTypeCheckerOptions>, value: u32);

        unsafe fn DebugTimeFunctionBodies(This: &CxxTypeCheckerOptions) -> bool;

        unsafe fn set_DebugTimeFunctionBodies(This: Pin<&mut CxxTypeCheckerOptions>, value: bool);

        unsafe fn DebugTimeExpressions(This: &CxxTypeCheckerOptions) -> bool;

        unsafe fn set_DebugTimeExpressions(This: Pin<&mut CxxTypeCheckerOptions>, value: bool);

        unsafe fn SkipFunctionBodies(This: &CxxTypeCheckerOptions) -> FunctionBodySkipping;

        unsafe fn set_SkipFunctionBodies(This: Pin<&mut CxxTypeCheckerOptions>, value: FunctionBodySkipping);

        unsafe fn DebugGenericSignatures(This: &CxxTypeCheckerOptions) -> bool;

        unsafe fn set_DebugGenericSignatures(This: Pin<&mut CxxTypeCheckerOptions>, value: bool);

        unsafe fn DebugConstraintSolver(This: &CxxTypeCheckerOptions) -> bool;

        unsafe fn set_DebugConstraintSolver(This: Pin<&mut CxxTypeCheckerOptions>, value: bool);

        unsafe fn DebugConstraintSolverAttempt(This: &CxxTypeCheckerOptions) -> bool;

        unsafe fn set_DebugConstraintSolverAttempt(This: Pin<&mut CxxTypeCheckerOptions>, value: bool);

        unsafe fn DebugConstraintSolverOnLines(This: &CxxTypeCheckerOptions) -> Vec<u32>;

        unsafe fn set_DebugConstraintSolverOnLines(This: Pin<&mut CxxTypeCheckerOptions>, value: &[u32]);

        unsafe fn DebugForbidTypecheckPrefix(This: &CxxTypeCheckerOptions) -> &CxxString;

        unsafe fn set_DebugForbidTypecheckPrefix(This: Pin<&mut CxxTypeCheckerOptions>, value: &str);

        unsafe fn SolverMemoryThreshold(This: &CxxTypeCheckerOptions) -> u32;

        unsafe fn set_SolverMemoryThreshold(This: Pin<&mut CxxTypeCheckerOptions>, value: u32);

        unsafe fn SolverBindingThreshold(This: &CxxTypeCheckerOptions) -> u32;

        unsafe fn set_SolverBindingThreshold(This: Pin<&mut CxxTypeCheckerOptions>, value: u32);

        unsafe fn SolverShrinkUnsolvedThreshold(This: &CxxTypeCheckerOptions) -> u32;

        unsafe fn set_SolverShrinkUnsolvedThreshold(This: Pin<&mut CxxTypeCheckerOptions>, value: u32);

        unsafe fn SolverDisableShrink(This: &CxxTypeCheckerOptions) -> bool;

        unsafe fn set_SolverDisableShrink(This: Pin<&mut CxxTypeCheckerOptions>, value: bool);

        unsafe fn EnableOperatorDesignatedTypes(This: &CxxTypeCheckerOptions) -> bool;

        unsafe fn set_EnableOperatorDesignatedTypes(This: Pin<&mut CxxTypeCheckerOptions>, value: bool);

        unsafe fn DisableConstraintSolverPerformanceHacks(This: &CxxTypeCheckerOptions) -> bool;

        unsafe fn set_DisableConstraintSolverPerformanceHacks(This: Pin<&mut CxxTypeCheckerOptions>, value: bool);

        unsafe fn EnableOneWayClosureParameters(This: &CxxTypeCheckerOptions) -> bool;

        unsafe fn set_EnableOneWayClosureParameters(This: Pin<&mut CxxTypeCheckerOptions>, value: bool);

        unsafe fn EnableMultiStatementClosureInference(This: &CxxTypeCheckerOptions) -> bool;

        unsafe fn set_EnableMultiStatementClosureInference(This: Pin<&mut CxxTypeCheckerOptions>, value: bool);

        unsafe fn PrintFullConvention(This: &CxxTypeCheckerOptions) -> bool;

        unsafe fn set_PrintFullConvention(This: Pin<&mut CxxTypeCheckerOptions>, value: bool);
    }
}

use self::ffi::CxxTypeCheckerOptions;
use crate::swift::FunctionBodySkipping;
use cxx::{CxxString, UniquePtr};

pub struct TypeCheckerOptions {
    pub(crate) ptr: UniquePtr<CxxTypeCheckerOptions>,
}

impl From<UniquePtr<CxxTypeCheckerOptions>> for TypeCheckerOptions {
    #[inline]
    fn from(ptr: UniquePtr<CxxTypeCheckerOptions>) -> Self {
        Self { ptr }
    }
}

impl TypeCheckerOptions {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }

    #[inline]
    pub unsafe fn warn_long_function_bodies(&self) -> u32 {
        let this = &self.ptr;
        self::ffi::WarnLongFunctionBodies(this)
    }

    #[inline]
    pub unsafe fn set_warn_long_function_bodies(&mut self, value: u32) {
        let this = self.ptr.pin_mut();
        self::ffi::set_WarnLongFunctionBodies(this, value)
    }

    #[inline]
    pub unsafe fn warn_long_expression_type_checking(&self) -> u32 {
        let this = &self.ptr;
        self::ffi::WarnLongExpressionTypeChecking(this)
    }

    #[inline]
    pub unsafe fn set_warn_long_expression_type_checking(&mut self, value: u32) {
        let this = self.ptr.pin_mut();
        self::ffi::set_WarnLongExpressionTypeChecking(this, value)
    }

    #[inline]
    pub unsafe fn expression_timeout_threshold(&self) -> u32 {
        let this = &self.ptr;
        self::ffi::ExpressionTimeoutThreshold(this)
    }

    #[inline]
    pub unsafe fn set_expression_timeout_threshold(&mut self, value: u32) {
        let this = self.ptr.pin_mut();
        self::ffi::set_ExpressionTimeoutThreshold(this, value)
    }

    #[inline]
    pub unsafe fn switch_checking_invocation_threshold(&self) -> u32 {
        let this = &self.ptr;
        self::ffi::SwitchCheckingInvocationThreshold(this)
    }

    #[inline]
    pub unsafe fn set_switch_checking_invocation_threshold(&mut self, value: u32) {
        let this = self.ptr.pin_mut();
        self::ffi::set_SwitchCheckingInvocationThreshold(this, value)
    }

    #[inline]
    pub unsafe fn debug_time_function_bodies(&self) -> bool {
        let this = &self.ptr;
        self::ffi::DebugTimeFunctionBodies(this)
    }

    #[inline]
    pub unsafe fn set_debug_time_function_bodies(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DebugTimeFunctionBodies(this, value)
    }

    #[inline]
    pub unsafe fn debug_time_expressions(&self) -> bool {
        let this = &self.ptr;
        self::ffi::DebugTimeExpressions(this)
    }

    #[inline]
    pub unsafe fn set_debug_time_expressions(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DebugTimeExpressions(this, value)
    }

    #[inline]
    pub unsafe fn skip_function_bodies(&self) -> FunctionBodySkipping {
        let this = &self.ptr;
        self::ffi::SkipFunctionBodies(this)
    }

    #[inline]
    pub unsafe fn set_skip_function_bodies(&mut self, value: FunctionBodySkipping) {
        let this = self.ptr.pin_mut();
        self::ffi::set_SkipFunctionBodies(this, value)
    }

    #[inline]
    pub unsafe fn debug_generic_signatures(&self) -> bool {
        let this = &self.ptr;
        self::ffi::DebugGenericSignatures(this)
    }

    #[inline]
    pub unsafe fn set_debug_generic_signatures(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DebugGenericSignatures(this, value)
    }

    #[inline]
    pub unsafe fn debug_constraint_solver(&self) -> bool {
        let this = &self.ptr;
        self::ffi::DebugConstraintSolver(this)
    }

    #[inline]
    pub unsafe fn set_debug_constraint_solver(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DebugConstraintSolver(this, value)
    }

    #[inline]
    pub unsafe fn debug_constraint_solver_attempt(&self) -> bool {
        let this = &self.ptr;
        self::ffi::DebugConstraintSolverAttempt(this)
    }

    #[inline]
    pub unsafe fn set_debug_constraint_solver_attempt(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DebugConstraintSolverAttempt(this, value)
    }

    #[inline]
    pub unsafe fn debug_constraint_solver_on_lines(&self) -> Vec<u32> {
        let this = &self.ptr;
        self::ffi::DebugConstraintSolverOnLines(this)
    }

    #[inline]
    pub unsafe fn set_debug_constraint_solver_on_lines(&mut self, value: &[u32]) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DebugConstraintSolverOnLines(this, value)
    }

    #[inline]
    pub unsafe fn debug_forbid_typecheck_prefix(&self) -> &CxxString {
        let this = &self.ptr;
        self::ffi::DebugForbidTypecheckPrefix(this)
    }

    #[inline]
    pub unsafe fn set_debug_forbid_typecheck_prefix(&mut self, value: &str) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DebugForbidTypecheckPrefix(this, value)
    }

    #[inline]
    pub unsafe fn solver_memory_threshold(&self) -> u32 {
        let this = &self.ptr;
        self::ffi::SolverMemoryThreshold(this)
    }

    #[inline]
    pub unsafe fn set_solver_memory_threshold(&mut self, value: u32) {
        let this = self.ptr.pin_mut();
        self::ffi::set_SolverMemoryThreshold(this, value)
    }

    #[inline]
    pub unsafe fn solver_binding_threshold(&self) -> u32 {
        let this = &self.ptr;
        self::ffi::SolverBindingThreshold(this)
    }

    #[inline]
    pub unsafe fn set_solver_binding_threshold(&mut self, value: u32) {
        let this = self.ptr.pin_mut();
        self::ffi::set_SolverBindingThreshold(this, value)
    }

    #[inline]
    pub unsafe fn solver_shrink_unsolved_threshold(&self) -> u32 {
        let this = &self.ptr;
        self::ffi::SolverShrinkUnsolvedThreshold(this)
    }

    #[inline]
    pub unsafe fn set_solver_shrink_unsolved_threshold(&mut self, value: u32) {
        let this = self.ptr.pin_mut();
        self::ffi::set_SolverShrinkUnsolvedThreshold(this, value)
    }

    #[inline]
    pub unsafe fn solver_disable_shrink(&self) -> bool {
        let this = &self.ptr;
        self::ffi::SolverDisableShrink(this)
    }

    #[inline]
    pub unsafe fn set_solver_disable_shrink(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_SolverDisableShrink(this, value)
    }

    #[inline]
    pub unsafe fn enable_operator_designated_types(&self) -> bool {
        let this = &self.ptr;
        self::ffi::EnableOperatorDesignatedTypes(this)
    }

    #[inline]
    pub unsafe fn set_enable_operator_designated_types(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_EnableOperatorDesignatedTypes(this, value)
    }

    #[inline]
    pub unsafe fn disable_constraint_solver_performance_hacks(&self) -> bool {
        let this = &self.ptr;
        self::ffi::DisableConstraintSolverPerformanceHacks(this)
    }

    #[inline]
    pub unsafe fn set_disable_constraint_solver_performance_hacks(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_DisableConstraintSolverPerformanceHacks(this, value)
    }

    #[inline]
    pub unsafe fn enable_one_way_closure_parameters(&self) -> bool {
        let this = &self.ptr;
        self::ffi::EnableOneWayClosureParameters(this)
    }

    #[inline]
    pub unsafe fn set_enable_one_way_closure_parameters(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_EnableOneWayClosureParameters(this, value)
    }

    #[inline]
    pub unsafe fn enable_multi_statement_closure_inference(&self) -> bool {
        let this = &self.ptr;
        self::ffi::EnableMultiStatementClosureInference(this)
    }

    #[inline]
    pub unsafe fn set_enable_multi_statement_closure_inference(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_EnableMultiStatementClosureInference(this, value)
    }

    #[inline]
    pub unsafe fn print_full_convention(&self) -> bool {
        let this = &self.ptr;
        self::ffi::PrintFullConvention(this)
    }

    #[inline]
    pub unsafe fn set_print_full_convention(&mut self, value: bool) {
        let this = self.ptr.pin_mut();
        self::ffi::set_PrintFullConvention(this, value)
    }
}
