use clang_importer_sys::swift::{clang_importer_options, ClangImporterOptions};

#[test]
fn new() {
    unsafe {
        let _ = ClangImporterOptions::new();
    }
}

#[test]
fn clang_path() {
    unsafe {
        let opts = ClangImporterOptions::new();
        cxx::let_cxx_string!(expected = "clang");
        assert_eq!(&*expected, opts.clang_path());
    }
}

#[test]
fn set_clang_path() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = "value";
        opts.set_clang_path(value);
        cxx::let_cxx_string!(expected = value);
        assert_eq!(&*expected, opts.clang_path());
    }
}

#[test]
fn module_cache_path() {
    unsafe {
        let opts = ClangImporterOptions::new();
        opts.clang_path();
    }
}

#[test]
fn set_module_cache_path() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = "value";
        opts.set_module_cache_path(value);
        cxx::let_cxx_string!(expected = value);
        assert_eq!(&*expected, opts.module_cache_path());
    }
}

#[test]
fn extra_args() {
    unsafe {
        let opts = ClangImporterOptions::new();
        opts.extra_args();
    }
}

#[test]
fn set_extra_args_empty() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        {
            let foo = String::from("foo");
            let foo = &foo[0 .. 1];
            let bar = String::from("bar");
            let bar = &bar[0 .. 1];
            let qux = String::from("qux");
            let qux = &qux[0 .. 1];
            opts.set_extra_args(&[foo, bar, qux]);
        }
        cxx::let_cxx_string!(foo = "f");
        cxx::let_cxx_string!(bar = "b");
        cxx::let_cxx_string!(qux = "q");
        assert_eq!(
            &[&*foo, &*bar, &*qux],
            &*opts.extra_args().into_iter().collect::<Vec<_>>()
        );
    }
}

#[test]
fn set_extra_args_overwrite_extend_additional() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        opts.set_extra_args(&["a", "b"]);
        cxx::let_cxx_string!(a = "a");
        cxx::let_cxx_string!(b = "b");
        assert_eq!(&[&*a, &*b], &*opts.extra_args().into_iter().collect::<Vec<_>>());
        opts.set_extra_args(&["x", "y", "z"]);
        cxx::let_cxx_string!(x = "x");
        cxx::let_cxx_string!(y = "y");
        cxx::let_cxx_string!(z = "z");
        assert_eq!(&[&*x, &*y, &*z], &*opts.extra_args().into_iter().collect::<Vec<_>>());
    }
}

#[test]
fn set_extra_args_overwrite_exact() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        opts.set_extra_args(&["a", "b", "c"]);
        cxx::let_cxx_string!(a = "a");
        cxx::let_cxx_string!(b = "b");
        cxx::let_cxx_string!(c = "c");
        assert_eq!(&[&*a, &*b, &*c], &*opts.extra_args().into_iter().collect::<Vec<_>>());
        opts.set_extra_args(&["x", "y", "z"]);
        cxx::let_cxx_string!(x = "x");
        cxx::let_cxx_string!(y = "y");
        cxx::let_cxx_string!(z = "z");
        assert_eq!(&[&*x, &*y, &*z], &*opts.extra_args().into_iter().collect::<Vec<_>>());
    }
}

#[test]
fn set_extra_args_overwrite_erase_trailing() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        opts.set_extra_args(&["x", "y", "z"]);
        cxx::let_cxx_string!(x = "x");
        cxx::let_cxx_string!(y = "y");
        cxx::let_cxx_string!(z = "z");
        assert_eq!(&[&*x, &*y, &*z], &*opts.extra_args().into_iter().collect::<Vec<_>>());
        opts.set_extra_args(&["a", "b"]);
        cxx::let_cxx_string!(a = "a");
        cxx::let_cxx_string!(b = "b");
        assert_eq!(&[&*a, &*b], &*opts.extra_args().into_iter().collect::<Vec<_>>());
    }
}

#[test]
fn override_resource_dir() {
    unsafe {
        let opts = ClangImporterOptions::new();
        opts.override_resource_dir();
    }
}

#[test]
fn set_override_resource_dir() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = "value";
        opts.set_override_resource_dir(value);
        cxx::let_cxx_string!(expected = value);
        assert_eq!(&*expected, opts.override_resource_dir());
    }
}

#[test]
fn target_cpu() {
    unsafe {
        let opts = ClangImporterOptions::new();
        opts.target_cpu();
    }
}

#[test]
fn set_target_cpu() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = "value";
        opts.set_target_cpu(value);
        cxx::let_cxx_string!(expected = value);
        assert_eq!(&*expected, opts.target_cpu());
    }
}

#[test]
fn index_store_path() {
    unsafe {
        let opts = ClangImporterOptions::new();
        opts.index_store_path();
    }
}

#[test]
fn set_index_store_path() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = "value";
        opts.set_index_store_path(value);
        cxx::let_cxx_string!(expected = value);
        assert_eq!(&*expected, opts.index_store_path());
    }
}

#[test]
fn bridging_header() {
    unsafe {
        let opts = ClangImporterOptions::new();
        opts.bridging_header();
    }
}

#[test]
fn set_bridging_header() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = "value";
        opts.set_bridging_header(value);
        cxx::let_cxx_string!(expected = value);
        assert_eq!(&*expected, opts.bridging_header());
    }
}

#[test]
fn precompiled_header_output_dir() {
    unsafe {
        let opts = ClangImporterOptions::new();
        opts.precompiled_header_output_dir();
    }
}

#[test]
fn set_precompiled_header_output_dir() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = "value";
        opts.set_precompiled_header_output_dir(value);
        cxx::let_cxx_string!(expected = value);
        assert_eq!(&*expected, opts.precompiled_header_output_dir());
    }
}

#[test]
fn optimization() {
    unsafe {
        let opts = ClangImporterOptions::new();
        opts.optimization();
    }
}

#[test]
fn set_optimization() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = "value";
        opts.set_optimization(value);
        cxx::let_cxx_string!(expected = value);
        assert_eq!(&*expected, opts.optimization());
    }
}

#[test]
fn pch_disable_validation() {
    unsafe {
        let opts = ClangImporterOptions::new();
        assert_eq!(false, opts.pch_disable_validation());
    }
}

#[test]
fn set_pch_disable_validation() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = true;
        opts.set_pch_disable_validation(value);
        let expected = value;
        assert_eq!(expected, opts.pch_disable_validation());
    }
}

#[test]
fn mode() {
    unsafe {
        let opts = ClangImporterOptions::new();
        assert_eq!(clang_importer_options::Modes::Normal, opts.mode());
    }
}

#[test]
fn set_mode() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        assert_eq!(clang_importer_options::Modes::Normal, opts.mode());
        let value = clang_importer_options::Modes::PrecompiledModule;
        opts.set_mode(value);
        let expected = value;
        assert_eq!(expected, opts.mode());
    }
}

#[test]
fn detailed_preprocessing_record() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        assert_eq!(false, opts.detailed_preprocessing_record());
    }
}

#[test]
fn set_detailed_preprocessing_record() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = true;
        opts.set_detailed_preprocessing_record(value);
        let expected = value;
        assert_eq!(expected, opts.detailed_preprocessing_record());
    }
}

#[test]
fn dump_clang_diagnostics() {
    unsafe {
        let opts = ClangImporterOptions::new();
        assert_eq!(false, opts.dump_clang_diagnostics());
    }
}

#[test]
fn set_dump_clang_diagnostics() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = true;
        opts.set_dump_clang_diagnostics(value);
        let expected = value;
        assert_eq!(expected, opts.dump_clang_diagnostics());
    }
}

#[test]
fn import_forward_declarations() {
    unsafe {
        let opts = ClangImporterOptions::new();
        assert_eq!(false, opts.import_forward_declarations());
    }
}

#[test]
fn set_import_forward_declarations() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = true;
        opts.set_import_forward_declarations(value);
        let expected = value;
        assert_eq!(expected, opts.import_forward_declarations());
    }
}

#[test]
fn disable_swift_bridge_attr() {
    unsafe {
        let opts = ClangImporterOptions::new();
        assert_eq!(false, opts.disable_swift_bridge_attr());
    }
}

#[test]
fn set_disable_swift_bridge_attr() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = true;
        opts.set_disable_swift_bridge_attr(value);
        let expected = value;
        assert_eq!(expected, opts.disable_swift_bridge_attr());
    }
}

#[test]
fn disable_overlay_modules() {
    unsafe {
        let opts = ClangImporterOptions::new();
        assert_eq!(false, opts.disable_overlay_modules());
    }
}

#[test]
fn set_disable_overlay_modules() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = true;
        opts.set_disable_overlay_modules(value);
        let expected = value;
        assert_eq!(expected, opts.disable_overlay_modules());
    }
}

#[test]
fn enable_clang_spi() {
    unsafe {
        let opts = ClangImporterOptions::new();
        assert_eq!(true, opts.enable_clang_spi());
    }
}

#[test]
fn set_enable_clang_spi() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = true;
        opts.set_enable_clang_spi(value);
        let expected = value;
        assert_eq!(expected, opts.enable_clang_spi());
    }
}

#[test]
fn debugger_support() {
    unsafe {
        let opts = ClangImporterOptions::new();
        assert_eq!(false, opts.debugger_support());
    }
}

#[test]
fn set_debugger_support() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = true;
        opts.set_debugger_support(value);
        let expected = value;
        assert_eq!(expected, opts.debugger_support());
    }
}

#[test]
fn disable_source_import() {
    unsafe {
        let opts = ClangImporterOptions::new();
        assert_eq!(false, opts.disable_source_import());
    }
}

#[test]
fn set_disable_source_import() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = true;
        opts.set_disable_source_import(value);
        let expected = value;
        assert_eq!(expected, opts.disable_source_import());
    }
}

#[test]
fn extra_args_only() {
    unsafe {
        let opts = ClangImporterOptions::new();
        assert_eq!(false, opts.extra_args_only());
    }
}

#[test]
fn set_extra_args_only() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        let value = true;
        opts.set_extra_args_only(value);
        let expected = value;
        assert_eq!(expected, opts.extra_args_only());
    }
}

#[test]
fn get_pch_hash_components() {
    unsafe {
        let opts = ClangImporterOptions::new();
        let _ = opts.get_pch_hash_components();
    }
}

#[test]
fn get_remapped_extra_args_unchanged() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        opts.set_extra_args(&["-unmapped", "-another=unmapped"]);
        let remapped = opts.get_remapped_extra_args(|_| "remapped".into());
        cxx::let_cxx_string!(arg0 = "-unmapped");
        cxx::let_cxx_string!(arg1 = "-another=unmapped");
        assert_eq!(vec![&*arg0, &*arg1], remapped.into_iter().collect::<Vec<_>>());
    }
}

#[test]
fn get_remapped_extra_args_changed() {
    unsafe {
        let mut opts = ClangImporterOptions::new();
        opts.set_extra_args(&[
            "-unmapped",
            "-another=unmapped",
            "-I",
            "some/path",
            "-ivfsoverlay",
            "another/path",
        ]);
        let remapped = opts.get_remapped_extra_args(|_| "remapped".into());
        cxx::let_cxx_string!(arg0 = "-unmapped");
        cxx::let_cxx_string!(arg1 = "-another=unmapped");
        cxx::let_cxx_string!(arg2 = "-I");
        cxx::let_cxx_string!(arg3 = "remapped");
        cxx::let_cxx_string!(arg4 = "-ivfsoverlay");
        cxx::let_cxx_string!(arg5 = "remapped");
        assert_eq!(
            vec![&*arg0, &*arg1, &*arg2, &*arg3, &*arg4, &*arg5],
            remapped.into_iter().collect::<Vec<_>>()
        );
    }
}
