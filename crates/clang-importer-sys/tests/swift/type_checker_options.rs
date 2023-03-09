use clang_importer_sys::swift::{FunctionBodySkipping, TypeCheckerOptions};

#[test]
fn new() {
    unsafe {
        let _ = TypeCheckerOptions::new();
    }
}

#[test]
fn warn_long_function_bodies() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(0, opts.warn_long_function_bodies());
    }
}

#[test]
fn set_warn_long_function_bodies() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_warn_long_function_bodies(6);
        assert_eq!(6, opts.warn_long_function_bodies());
    }
}

#[test]
fn warn_long_expression_type_checking() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(0, opts.warn_long_expression_type_checking());
    }
}

#[test]
fn set_warn_long_expression_type_checking() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_warn_long_expression_type_checking(6);
        assert_eq!(6, opts.warn_long_expression_type_checking());
    }
}

#[test]
fn expression_timeout_threshold() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(600, opts.expression_timeout_threshold());
    }
}

#[test]
fn set_expression_timeout_threshold() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_expression_timeout_threshold(6);
        assert_eq!(6, opts.expression_timeout_threshold());
    }
}

#[test]
fn switch_checking_invocation_threshold() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(200000, opts.switch_checking_invocation_threshold());
    }
}

#[test]
fn set_switch_checking_invocation_threshold() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_switch_checking_invocation_threshold(6);
        assert_eq!(6, opts.switch_checking_invocation_threshold());
    }
}

#[test]
fn debug_time_function_bodies() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(false, opts.debug_time_function_bodies());
    }
}

#[test]
fn set_debug_time_function_bodies() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_debug_time_function_bodies(true);
        assert_eq!(true, opts.debug_time_function_bodies());
    }
}

#[test]
fn debug_time_expressions() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(false, opts.debug_time_expressions());
    }
}

#[test]
fn set_debug_time_expressions() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_debug_time_expressions(true);
        assert_eq!(true, opts.debug_time_expressions());
    }
}

#[test]
fn skip_function_bodies() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(FunctionBodySkipping::None, opts.skip_function_bodies());
    }
}

#[test]
fn set_skip_function_bodies() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_skip_function_bodies(FunctionBodySkipping::NonInlinable);
        assert_eq!(FunctionBodySkipping::NonInlinable, opts.skip_function_bodies());
    }
}

#[test]
fn debug_generic_signatures() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(false, opts.debug_generic_signatures());
    }
}

#[test]
fn set_debug_generic_signatures() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_debug_generic_signatures(true);
        assert_eq!(true, opts.debug_generic_signatures());
    }
}

#[test]
fn debug_constraint_solver() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(false, opts.debug_constraint_solver());
    }
}

#[test]
fn set_debug_constraint_solver() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_debug_constraint_solver(true);
        assert_eq!(true, opts.debug_constraint_solver());
    }
}

#[test]
fn debug_constraint_solver_attempt() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(false, opts.debug_constraint_solver_attempt());
    }
}

#[test]
fn set_debug_constraint_solver_attempt() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_debug_constraint_solver_attempt(true);
        assert_eq!(true, opts.debug_constraint_solver_attempt());
    }
}

#[test]
fn debug_constraint_solver_on_lines() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        let expected = Vec::<u32>::new();
        assert_eq!(expected, opts.debug_constraint_solver_on_lines());
    }
}

#[test]
fn set_debug_constraint_solver_on_lines_extend_additional() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        let expected = Vec::<u32>::new();
        assert_eq!(expected, opts.debug_constraint_solver_on_lines());
        let value = &[3, 0, 2, 1];
        opts.set_debug_constraint_solver_on_lines(value);
        assert_eq!(value, &*opts.debug_constraint_solver_on_lines());
        let value = &[8, 3, 9, 1, 5, 7];
        opts.set_debug_constraint_solver_on_lines(value);
        assert_eq!(value, &*opts.debug_constraint_solver_on_lines());
    }
}

#[test]
fn set_debug_constraint_solver_on_lines_overwrite_exact() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        let expected = Vec::<u32>::new();
        assert_eq!(expected, opts.debug_constraint_solver_on_lines());
        let value = &[3, 0, 2, 1];
        opts.set_debug_constraint_solver_on_lines(value);
        assert_eq!(value, &*opts.debug_constraint_solver_on_lines());
        let value = &[8, 3, 9, 1];
        opts.set_debug_constraint_solver_on_lines(value);
        assert_eq!(value, &*opts.debug_constraint_solver_on_lines());
    }
}

#[test]
fn set_debug_constraint_solver_on_lines_erase_trailing() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        let expected = Vec::<u32>::new();
        assert_eq!(expected, opts.debug_constraint_solver_on_lines());
        let value = &[8, 3, 9, 1, 5, 7];
        opts.set_debug_constraint_solver_on_lines(value);
        assert_eq!(value, &*opts.debug_constraint_solver_on_lines());
        let value = &[3, 0, 2, 1];
        opts.set_debug_constraint_solver_on_lines(value);
        assert_eq!(value, &*opts.debug_constraint_solver_on_lines());
    }
}

#[test]
fn debug_forbid_typecheck_prefix() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        cxx::let_cxx_string!(expected = "");
        assert_eq!(&*expected, opts.debug_forbid_typecheck_prefix());
    }
}

#[test]
fn set_debug_forbid_typecheck_prefix() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        let value = "string";
        opts.set_debug_forbid_typecheck_prefix(value);
        cxx::let_cxx_string!(expected = value);
        assert_eq!(&*expected, opts.debug_forbid_typecheck_prefix());
    }
}

#[test]
fn solver_memory_threshold() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(512 * 1024 * 1024, opts.solver_memory_threshold());
    }
}

#[test]
fn set_solver_memory_threshold() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_solver_memory_threshold(6);
        assert_eq!(6, opts.solver_memory_threshold());
    }
}

#[test]
fn solver_binding_threshold() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(1024 * 1024, opts.solver_binding_threshold());
    }
}

#[test]
fn set_solver_binding_threshold() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_solver_binding_threshold(6);
        assert_eq!(6, opts.solver_binding_threshold());
    }
}

#[test]
fn solver_shrink_unsolved_threshold() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(10, opts.solver_shrink_unsolved_threshold());
    }
}

#[test]
fn set_solver_shrink_unsolved_threshold() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_solver_shrink_unsolved_threshold(6);
        assert_eq!(6, opts.solver_shrink_unsolved_threshold());
    }
}

#[test]
fn solver_disable_shrink() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(false, opts.solver_disable_shrink());
    }
}

#[test]
fn set_solver_disable_shrink() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_solver_disable_shrink(true);
        assert_eq!(true, opts.solver_disable_shrink());
    }
}

#[test]
fn enable_operator_designated_types() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(false, opts.enable_operator_designated_types());
    }
}

#[test]
fn set_enable_operator_designated_types() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_enable_operator_designated_types(true);
        assert_eq!(true, opts.enable_operator_designated_types());
    }
}

#[test]
fn disable_constraint_solver_performance_hacks() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(false, opts.disable_constraint_solver_performance_hacks());
    }
}

#[test]
fn set_disable_constraint_solver_performance_hacks() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_disable_constraint_solver_performance_hacks(true);
        assert_eq!(true, opts.disable_constraint_solver_performance_hacks());
    }
}

#[test]
fn enable_one_way_closure_parameters() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(false, opts.enable_one_way_closure_parameters());
    }
}

#[test]
fn set_enable_one_way_closure_parameters() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_enable_one_way_closure_parameters(true);
        assert_eq!(true, opts.enable_one_way_closure_parameters());
    }
}

#[test]
fn enable_multi_statement_closure_inference() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(true, opts.enable_multi_statement_closure_inference());
    }
}

#[test]
fn set_enable_multi_statement_closure_inference() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_enable_multi_statement_closure_inference(false);
        assert_eq!(false, opts.enable_multi_statement_closure_inference());
    }
}

#[test]
fn print_full_convention() {
    unsafe {
        let opts = TypeCheckerOptions::new();
        assert_eq!(false, opts.print_full_convention());
    }
}

#[test]
fn set_print_full_convention() {
    unsafe {
        let mut opts = TypeCheckerOptions::new();
        opts.set_print_full_convention(true);
        assert_eq!(true, opts.print_full_convention());
    }
}
