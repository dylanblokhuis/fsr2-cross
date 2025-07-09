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

    // run the command "cmake -DFFX_FSR2_API_DX12=OFF -DFFX_FSR2_API_VK=ON .." in fsr2/src/ffx-fsr2-api
    let info = std::process::Command::new("cmake")
        .arg("-DFFX_FSR2_API_DX12=OFF")
        .arg("-DFFX_FSR2_API_VK=ON")
        .arg("-BFidelityFX-FSR2/src/ffx-fsr2-api/build")
        .arg("-SFidelityFX-FSR2/src/ffx-fsr2-api")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .expect("Failed to run cmake");

    if !info.success() {
        panic!("CMake command failed with status: {}", info);
    }

    // run make to build the project
    let info = std::process::Command::new("make")
        .arg("-CFidelityFX-FSR2/src/ffx-fsr2-api/build")
        .arg("-j4") // Use 4 threads for building
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .expect("Failed to run make");

    if !info.success() {
        panic!("Make command failed with status: {}", info);
    }

    println!("cargo:rerun-if-changed=FidelityFX-FSR2/src/ffx-fsr2-api/ffx_fsr2.h");
    println!("cargo:rerun-if-changed=FidelityFX-FSR2/src/ffx-fsr2-api/vk/ffx_fsr2_vk.h");

    println!("cargo:rustc-link-search=native=FidelityFX-FSR2/src/ffx-fsr2-api/build");
    println!("cargo:rustc-link-search=native=FidelityFX-FSR2/src/ffx-fsr2-api/build/vk");

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
