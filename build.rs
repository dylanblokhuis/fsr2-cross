use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("ffx-fsr2-build");
    let source_dir = PathBuf::from("FidelityFX-FSR2/src/ffx-fsr2-api");

    // Run cmake to configure the project
    let cmake_status = Command::new("cmake")
        .arg("-DFFX_FSR2_API_DX12=OFF")
        .arg("-DFFX_FSR2_API_VK=ON")
        .arg(format!("-B{}", build_dir.display()))
        .arg(format!("-S{}", source_dir.display()))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to run cmake");

    if !cmake_status.success() {
        panic!("CMake configuration failed with status: {}", cmake_status);
    }

    // Run make to build the project
    let make_status = Command::new("make")
        .arg(format!("-C{}", build_dir.display()))
        .arg("-j4")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to run make");

    if !make_status.success() {
        panic!("Make command failed with status: {}", make_status);
    }

    // Inform Cargo to re-run if headers change
    println!("cargo:rerun-if-changed={}/ffx_fsr2.h", source_dir.display());
    println!(
        "cargo:rerun-if-changed={}/vk/ffx_fsr2_vk.h",
        source_dir.display()
    );

    // Link paths
    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-search=native={}/vk", build_dir.display());

    // Determine which library to link based on target arch
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match target_arch.as_str() {
        "arm" | "aarch64" => {
            println!("cargo:rustc-link-lib=static=ffx_fsr2_api_arm64");
            println!("cargo:rustc-link-lib=static=ffx_fsr2_api_vk_arm64");
        }
        "x86_64" => {
            println!("cargo:rustc-link-lib=static=ffx_fsr2_api_x86_64");
            println!("cargo:rustc-link-lib=static=ffx_fsr2_api_vk_x86_64");
        }
        _ => {
            panic!("Unsupported architecture: {}", target_arch);
        }
    }

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dynamic=stdc++");
    }
}
