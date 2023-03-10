# framework-translator

An Objective-C framework translator for Rust using Swift's [ClangImporter](https://github.com/apple/swift/tree/main/lib/ClangImporter)

## Status

This project is still in a very early stage and is not yet usable. The current focus is on building out the necessary bindings to the `ClangImporter` and related libraries. Once that is closer to completion, work will begin on the CLI frontend.

The eventual goal is to be able to produce something suitable for usage with the [objc2](https://github.com/madsmtm/objc2) project that addresses some of the issues with the current [header-translator](https://github.com/madsmtm/objc2/tree/master/crates/header-translator). See [this issue](https://github.com/madsmtm/objc2/issues/345) for further details.

## Building

1. Clone the Swift project and related repos by following the instructions [here](https://github.com/apple/swift/blob/main/docs/HowToGuides/GettingStarted.md#cloning-the-project).
2. Run `./utils/update-checkout --tag swift-5.7.3-RELEASE` (from within the `swift` directory) to sync the repositories to the `5.7.3` release tag.
3. In a directory adjacent to `swift-project` (see instructions above), clone this project to a directory `framework-translator`.
4. You should end up with a directory structure like this:

```
├── framework-translator
└── swift-project
    ├── cmark
    ├── llvm-project
    |   ├── clang
    |   │   └── include
    |   └── llvm
    |       └── include
    └── swift
        └── include
            └── swift
                └── ClangImporter
```

5. Next, build the Swift toolchain by following the instructions [here](https://github.com/apple/swift/blob/main/docs/HowToGuides/GettingStarted.md#building-the-project-for-the-first-time), using the command below.

_If building on macOS, also include the flag `--swift-darwin-supported-archs "$(uname -m)"`. Additionally, read the notes in the instructions above about `sccache` and `--bootstrapping=hosttools` (i.e., ensure you have `sccache` installed and an existing swift toolchain installed)._

Run the following command to build the toolchain:

```
./utils/build-script --skip-build-benchmarks --skip-ios --skip-watchos --skip-tvos --sccache --release-debuginfo --swift-disable-dead-stripping --skip-early-swift-driver --bootstrapping=hosttools
```

6. You should end up with a new `build` subdirectory so that the directory structure now looks like this:

```
├── framework-translator
└── swift-project
    ├── build
    │   └── Ninja-RelWithDebInfoAssert
    │       ├── cmark-linux-x86_64
    │       │   └── src
    │       ├── llvm-linux-x86_64
    |       │   ├── include
    |       │   ├── lib
    │       │   └── tools
    |       │       └── clang
    |       │           └── include
    │       └── swift-linux-x86_64
    |           ├── include
    |           └── lib
    ├── cmark
    ├── llvm-project
    |   ├── clang
    |   │   └── include
    |   └── llvm
    |       └── include
    └── swift
        └── include
            └── swift
                └── ClangImporter
```

7. Finally, build the crate with `cargo build`, which will link against the previously built `ClangImporter` library.
