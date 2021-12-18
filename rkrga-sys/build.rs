#[cfg(feature = "use-bindgen")]
const DEFAULT_RKRGA_INCLUDE_DIR: &str = "/opt/fullv/2021.02.7-rklaser1/staging/usr/include/rga";
#[cfg(feature = "use-bindgen")]
const DEFAULT_RKRGA_SYSROOT_DIR: &str = "/opt/fullv/2021.02.7-rklaser1/staging";

#[cfg(feature = "use-bindgen")]
fn generate_bindings() {
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    println!("cargo:rerun-if-env-changed=RKRGA_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=RKRGA_SYSROOT_DIR");

    let rkrga_include_dir =
        env::var("RKRGA_INCLUDE_DIR").unwrap_or_else(|_| DEFAULT_RKRGA_INCLUDE_DIR.into());
    let rkrga_sysroot_dir =
        env::var("RKRGA_SYSROOT_DIR").unwrap_or_else(|_| DEFAULT_RKRGA_SYSROOT_DIR.into());

    let wrapper_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("wrapper.h");
    let wrapper_path = wrapper_path.to_str().unwrap();
    let mut wrapper = File::create(wrapper_path).unwrap();
    writeln!(wrapper, "#include <RgaApi.h>").unwrap();

    let bindings = bindgen::Builder::default()
        .header(wrapper_path)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .anon_fields_prefix("un")
        .derive_debug(true)
        .impl_debug(false)
        .derive_default(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .impl_partialeq(true)
        .allowlist_function("c_Rk.*")
        .allowlist_function("rga_.*")
        .allowlist_function("Rga.*")
        .allowlist_type("drm_.*")
        .allowlist_type("rga_.*")
        .allowlist_type("Rga.*")
        .allowlist_type("RGA_.*")
        .allowlist_var("HAL_.*")
        .allowlist_var("RGA_.*")
        .allowlist_var("RK_.*")
        .clang_arg(format!("-I{}", rkrga_include_dir))
        .clang_arg(format!("--sysroot={}", rkrga_sysroot_dir))
        // .parse_callbacks(Box::new(MyParseCallbacks::default()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "use-bindgen")]
    generate_bindings();

    println!("cargo:rustc-link-lib=dylib=rga");
}
