# framework-translator

An Objective-C framework translator for Rust using Swift's [ClangImporter](https://github.com/apple/swift/tree/main/lib/ClangImporter)

## Building

1. Clone the Swift project and related repos by following the instructions [here](https://github.com/apple/swift/blob/main/docs/HowToGuides/GettingStarted.md#cloning-the-project).
2. In a directory adjacent to `swift-project` (see instructions above), clone this project to a directory `framework-translator`.
3. You should end up with a directory structure like this:

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

4. Next, build the Swift toolchain by following the instructions [here](https://github.com/apple/swift/blob/main/docs/HowToGuides/GettingStarted.md#building-the-project-for-the-first-time).
5. You should end up with a new `build` subdirectory so that the directory structure now looks like this:

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

6. Next, build the crate with `cargo build`, which will link against the previously built `ClangImporter` library.
