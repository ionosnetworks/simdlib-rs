use std::borrow::Borrow;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

fn run_command_or_fail<P, S>(dir: &str, cmd: P, args: &[S])
where
    P: AsRef<Path>,
    S: Borrow<str> + AsRef<OsStr>,
{
    let cmd = cmd.as_ref();
    let cmd = if cmd.components().count() > 1 && cmd.is_relative() {
        // If `cmd` is a relative path (and not a bare command that should be
        // looked up in PATH), absolutize it relative to `dir`, as otherwise the
        // behavior of std::process::Command is undefined.
        // https://github.com/rust-lang/rust/issues/37868
        PathBuf::from(dir)
            .join(cmd)
            .canonicalize()
            .expect("canonicalization failed")
    } else {
        PathBuf::from(cmd)
    };
    eprintln!(
        "Running command: \"{} {}\" in dir: {}",
        cmd.display(),
        args.join(" "),
        dir
    );
    let ret = Command::new(cmd).current_dir(dir).args(args).status();
    match ret.map(|status| (status.success(), status.code())) {
        Ok((true, _)) => (),
        Ok((false, Some(c))) => panic!("Command failed with error code {}", c),
        Ok((false, None)) => panic!("Command got killed"),
        Err(e) => panic!("Command failed with error: {}", e),
    }
}

fn main() {
    if !Path::new("Simd/LICENSE").exists() {
        eprintln!("Setting up submodules");
        run_command_or_fail("../", "git", &["submodule", "update", "--init"]);
    }
    eprintln!("Building and linking Simd statically");
    build_simdlibrary();
}

fn build_simdlibrary() {
    let mut config = cmake::Config::new("Simd/prj/cmake");

    config
        .define("SIMD_TOOLCHAIN", "")
        .define("SIMD_TARGET", "")
        .no_build_target(true);

    if env::var("CARGO_FEATURE_AVX512").is_ok() {
        config.define("SIMD_AVX512", "1");
    } else {
        config.define("SIMD_AVX512", "0");
    }

    if env::var("CARGO_FEATURE_AVX512VNNI").is_ok() {
        config.define("SIMD_AVX512VNNI", "1");
    } else {
        config.define("SIMD_AVX512VNNI", "0");
    }

    if env::var("CARGO_FEATURE_AVX512BF16").is_ok() {
        config.define("SIMD_AVX512BF16", "1");
    } else {
        config.define("SIMD_AVX512BF16", "0");
    }

    if env::var("CARGO_FEATURE_AMX").is_ok() {
        config.define("SIMD_AMX", "1");
    } else {
        config.define("SIMD_AMX", "0");
    }

    if env::var("CARGO_FEATURE_TEST").is_ok() {
        config.define("SIMD_TEST", "1");
    } else {
        config.define("SIMD_TEST", "0");
    }

    if env::var("CARGO_FEATURE_PERF").is_ok() {
        config.define("SIMD_PERF", "1");
    } else {
        config.define("SIMD_PERF", "0");
    }

    if env::var("CARGO_FEATURE_SHARED").is_ok() {
        config.define("SIMD_SHARED", "1");
    } else {
        config.define("SIMD_SHARED", "0");
    }

    if env::var("CARGO_FEATURE_GET_VERSION").is_ok() {
        config.define("SIMD_GET_VERSION", "1");
    } else {
        config.define("SIMD_GET_VERSION", "0");
    }

    if env::var("CARGO_FEATURE_SYNET").is_ok() {
        config.define("SIMD_SYNET", "1");
    } else {
        config.define("SIMD_SYNET", "0");
    }

    if env::var("CARGO_FEATURE_SINT8_DEBUG").is_ok() {
        config.define("SIMD_INT8_DEBUG", "1");
    } else {
        config.define("SIMD_INT8_DEBUG", "0");
    }

    if env::var("CARGO_FEATURE_HIDE").is_ok() {
        config.define("SIMD_HIDE", "1");
    } else {
        config.define("SIMD_HIDE", "0");
    }

    if let Ok(system_name) = env::var("CMAKE_SYSTEM_NAME") {
        config.define("CMAKE_SYSTEM_NAME", system_name);
    }

    println!("Configuring and compiling simdlibrary");
    let dst = config.build();

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=static=Simd");
}
