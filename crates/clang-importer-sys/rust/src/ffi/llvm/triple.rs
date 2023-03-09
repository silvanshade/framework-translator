#[cxx::bridge]
pub(crate) mod ffi {
    // NOTE: needed for the following enums (due to "last" cases)
    #![allow(unreachable_patterns)]

    #[namespace = "llvm"]
    extern "C++" {
        include!("llvm/ADT/Triple.h");

        #[cxx_name = "Triple"]
        type CxxTriple;
    }

    #[namespace = "rust::llvm"]
    #[derive(Debug)]
    #[repr(u32)]
    enum ArchType {
        UnknownArch,
        arm,
        armeb,
        aarch64,
        aarch64_be,
        aarch64_32,
        arc,
        avr,
        bpfel,
        bpfeb,
        csky,
        hexagon,
        m68k,
        mips,
        mipsel,
        mips64,
        mips64el,
        msp430,
        ppc,
        ppcle,
        ppc64,
        ppc64le,
        r600,
        amdgcn,
        riscv32,
        riscv64,
        sparc,
        sparcv9,
        sparcel,
        systemz,
        tce,
        tcele,
        thumb,
        thumbeb,
        x86,
        x86_64,
        xcore,
        nvptx,
        nvptx64,
        le32,
        le64,
        amdil,
        amdil64,
        hsail,
        hsail64,
        spir,
        spir64,
        kalimba,
        shave,
        lanai,
        wasm32,
        wasm64,
        renderscript32,
        renderscript64,
        ve, // 54
        LastArchType = 54,
    }

    #[namespace = "rust::llvm"]
    #[derive(Debug)]
    #[repr(u32)]
    enum SubArchType {
        NoSubArch,
        ARMSubArch_v9_2a,
        ARMSubArch_v9_1a,
        ARMSubArch_v9,
        ARMSubArch_v8_7a,
        ARMSubArch_v8_6a,
        ARMSubArch_v8_5a,
        ARMSubArch_v8_4a,
        ARMSubArch_v8_3a,
        ARMSubArch_v8_2a,
        ARMSubArch_v8_1a,
        ARMSubArch_v8,
        ARMSubArch_v8r,
        ARMSubArch_v8m_baseline,
        ARMSubArch_v8m_mainline,
        ARMSubArch_v8_1m_mainline,
        ARMSubArch_v7,
        ARMSubArch_v7em,
        ARMSubArch_v7m,
        ARMSubArch_v7s,
        ARMSubArch_v7k,
        ARMSubArch_v7ve,
        ARMSubArch_v6,
        ARMSubArch_v6m,
        ARMSubArch_v6k,
        ARMSubArch_v6t2,
        ARMSubArch_v5,
        ARMSubArch_v5te,
        ARMSubArch_v4t,
        AArch64SubArch_arm64e,
        KalimbaSubArch_v3,
        KalimbaSubArch_v4,
        KalimbaSubArch_v5,
        MipsSubArch_r6,
        PPCSubArch_spe,
    }

    #[namespace = "rust::llvm"]
    #[derive(Debug)]
    #[repr(u32)]
    enum VendorType {
        UnknownVendor,
        Apple,
        PC,
        SCEI,
        Freescale,
        IBM,
        ImaginationTechnologies,
        MipsTechnologies,
        NVIDIA,
        CSR,
        Myriad,
        AMD,
        Mesa,
        SUSE,
        OpenEmbedded, // 14
        LastVendorType = 14,
    }

    #[namespace = "rust::llvm"]
    #[derive(Debug)]
    #[repr(u32)]
    enum OSType {
        UnknownOS,
        Ananas,
        CloudABI,
        Darwin,
        DragonFly,
        FreeBSD,
        Fuchsia,
        IOS,
        KFreeBSD,
        Linux,
        Lv2,
        MacOSX,
        NetBSD,
        OpenBSD,
        Solaris,
        Win32,
        ZOS,
        Haiku,
        Minix,
        RTEMS,
        NaCl,
        AIX,
        CUDA,
        NVCL,
        AMDHSA,
        PS4,
        ELFIAMCU,
        TvOS,
        WatchOS,
        Mesa3D,
        Contiki,
        AMDPAL,
        HermitCore,
        Hurd,
        WASI,
        Emscripten, // 35
        LastOSType = 35,
    }

    #[namespace = "rust::llvm"]
    #[derive(Debug)]
    #[repr(u32)]
    enum EnvironmentType {
        UnknownEnvironment,
        GNU,
        GNUABIN32,
        GNUABI64,
        GNUEABI,
        GNUEABIHF,
        GNUX32,
        GNUILP32,
        CODE16,
        EABI,
        EABIHF,
        Android,
        Musl,
        MuslEABI,
        MuslEABIHF,
        MuslX32,
        MSVC,
        Itanium,
        Cygnus,
        CoreCLR,
        Simulator,
        MacABI, // 21
        LastEnvironmentType = 21,
    }

    #[namespace = "rust::llvm"]
    #[derive(Debug)]
    #[repr(u32)]
    enum ObjectFormatType {
        UnknownObjectFormat,
        COFF,
        ELF,
        GOFF,
        MachO,
        Wasm,
        XCOFF,
    }

    extern "C++" {
        include!("cxx/llvm/Triple.hxx");

        type ArchType;

        type SubArchType;

        type VendorType;

        type OSType;

        type EnvironmentType;

        type ObjectFormatType;
    }

    #[namespace = "cxx::llvm::Triple"]
    extern "C++" {
        include!("cxx/llvm/Triple.hxx");

        #[namespace = "llvm"]
        #[cxx_name = "Twine"]
        type CxxTwine<'a> = crate::ffi::llvm::twine::ffi::CxxTwine<'a>;

        unsafe fn make() -> UniquePtr<CxxTriple>;

        unsafe fn from_twine(str: &CxxTwine<'_>) -> UniquePtr<CxxTriple>;

        unsafe fn from_arch_vendor_os(
            arch: &CxxTwine<'_>,
            vendor: &CxxTwine<'_>,
            os: &CxxTwine<'_>,
        ) -> UniquePtr<CxxTriple>;
    }
}

use self::ffi::CxxTriple;
use crate::llvm::Twine;
use cxx::UniquePtr;

pub struct Triple {
    pub(crate) ptr: UniquePtr<CxxTriple>,
}

impl From<&Twine<'_>> for Triple {
    #[inline]
    fn from(value: &Twine<'_>) -> Self {
        let ptr = unsafe { self::ffi::from_twine(&value.ptr) };
        Self { ptr }
    }
}

impl Triple {
    #[inline]
    pub unsafe fn new() -> Self {
        let ptr = self::ffi::make();
        Self { ptr }
    }

    #[inline]
    pub unsafe fn from_arch_vendor_os(arch: &Twine<'_>, vendor: &Twine<'_>, os: &Twine<'_>) -> Self {
        let ptr = self::ffi::from_arch_vendor_os(&arch.ptr, &vendor.ptr, &os.ptr);
        Self { ptr }
    }

    #[inline]
    pub unsafe fn from_twine(str: &Twine<'_>) -> Self {
        let ptr = self::ffi::from_twine(&str.ptr);
        Self { ptr }
    }
}
