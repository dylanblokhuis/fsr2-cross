// extern crate bindgen;

fn main() {
    // let bindings = bindgen::Builder::default()
    //     .header("fsr2/src/ffx-fsr2-api/ffx_fsr2.h")
    //     .header("fsr2/src/ffx-fsr2-api/vk/ffx_fsr2_vk.h")
    //     // .allowlist_function("ffxFsr.*")
    //     .allowlist_type("FfxFsr2.*")
    //     .allowlist_function("ffx.*")
    //     .allowlist_item("FFX.*")
    //     // .trust_clang_mangling(false)
    //     // .layout_tests(false)
    //     .generate()
    //     .unwrap();

    // bindings.write_to_file("src/ffx_fsr2.rs").unwrap();

    println!("cargo:rerun-if-changed=fsr2/src/ffx-fsr2-api/ffx_fsr2.h");
    println!("cargo:rerun-if-changed=fsr2/src/ffx-fsr2-api/vk/ffx_fsr2_vk.h");

    println!("cargo:rustc-link-search=native=fsr2/src/ffx-fsr2-api/build");
    println!("cargo:rustc-link-search=native=fsr2/src/ffx-fsr2-api/build/vk");

    // if arm then link to arm64
    if cfg!(target_arch = "arm") {
        println!("cargo:rustc-link-lib=static=ffx_fsr2_api_arm64");
        println!("cargo:rustc-link-lib=static=ffx_fsr2_api_vk_arm64");
    } else if cfg!(target_arch = "aarch64") {
        println!("cargo:rustc-link-lib=static=ffx_fsr2_api_arm64");
        println!("cargo:rustc-link-lib=static=ffx_fsr2_api_vk_arm64");
    } else if cfg!(target_arch = "x86_64") {
        println!("cargo:rustc-link-lib=static=ffx_fsr2_api_x86_64");
        println!("cargo:rustc-link-lib=static=ffx_fsr2_api_vk_x86_64");
    } else {
        panic!(
            "Unsupported architecture: {}",
            std::env::var("CARGO_CFG_TARGET_ARCH").unwrap()
        );
    }

    println!("cargo:rustc-link-lib=stdc++");
}
