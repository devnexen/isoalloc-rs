fn main() {
    let mut build = cc::Build::new();
    let prof = std::env::var("PROFILE").expect("there should be a profile");
    build.compiler("clang");
    build.include("isoalloc/include");
    build.files([
        "isoalloc/src/iso_alloc.c",
        "isoalloc/src/iso_alloc_util.c",
        "isoalloc/src/iso_alloc_interfaces.c",
        "isoalloc/src/iso_alloc_mem_tags.c",
        "isoalloc/src/iso_alloc_signal.c",
        "isoalloc/src/iso_alloc_random.c",
        "isoalloc/src/iso_alloc_profiler.c",
        "isoalloc/src/iso_alloc_sanity.c",
        "isoalloc/src/iso_alloc_search.c",
        "isoalloc/src/iso_alloc_printf.c",
        "isoalloc/src/libc_hook.c",
        "isoalloc/src/malloc_hook.c",
    ]);

    build.define("DISABLE_CANARY", "0");
    build.define("SANITIZE_CHUNKS", "1");
    build.define("FUZZ_MODE", "0");
    build.define("PERM_FREE_REALLOC", "0");
    build.define("MEMORY_TAGGING", "0");
    build.define("ABORT_NO_ENTROPY", "0");
    build.define("THREAD_SUPPORT", "1");
    build.define("USE_SPINLOCK", "0");
    build.define("STARTUP_MEM_USAGE", "0");
    build.define("MALLOC_HOOK", "1");
    build.define("HUGE_PAGES", "1");
    build.define("AUTO_CTOR_DTOR", "1");
    build.define("SCHED_GETCPU", "1");

    if cfg!(feature = "userfaultfd") {
        build.define("UNINIT_READ_SANITY", "1");
    }

    if cfg!(feature = "sanity") {
        build.define("ALLOC_SANITY", "1");
        build.define("MEMCPY_SANITY", "1");
        build.define("MEMSET_SANITY", "1");
        build.flag("-D_FORTIFY_SOURCE=0");
    }

    build.flag("-pthread");
    build.flag("-Wno-pointer-arith");
    build.flag("-Wno-gnu-zero-variadic-macro-arguments");
    build.flag("-Wno-format-pedantic");
    build.flag("-fstrict-aliasing");
    build.flag("-Wnostrict-aliasing");

    match prof.as_str() {
        "debug" => {
            build.define("RANDOMIZE_FREELIST", "1");
            build.debug(true);
        }
        "release" => {
            build.debug(false);
        }
        _ => (),
    }

    build.static_flag(true);
    build.compile("isoalloc");
}
