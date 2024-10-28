use glob::glob;
use std::path::PathBuf;

const IGNORED: &[&str] = &["examples", "tests", "ggml-cann", "ggml-kompute", "ggml-sycl", "ggml-vulkan", "ggml-cuda"];

const INCLUDE: &[&str] = &[
        "include",
        "BitNet/include",
        "BitNet/3rdparty/llama.cpp/include",
        "BitNet/3rdparty/llama.cpp/common",
        "BitNet/3rdparty/llama.cpp/ggml/include",
        "BitNet/3rdparty/llama.cpp/ggml/src",
        "/opt/homebrew/opt/openblas/include"
];

fn main() {
    println!("cargo:include=/opt/homebrew/opt/openblas/include");

  // Collect all `.cpp` files from `BitNet` and subdirectories
    let cpp_files: Vec<PathBuf> = glob("BitNet/**/*.cpp")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter(|path| path.is_file())
        .filter(|path| !IGNORED.iter().any(|ig| path.to_string_lossy().contains(ig)))
        .collect();

    let c_files: Vec<PathBuf> = glob("BitNet/**/*.c")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter(|path| path.is_file())
        .filter(|path| !IGNORED.iter().any(|ig| path.to_string_lossy().contains(ig)))
        .collect();

    cxx_build::bridge("src/main.rs")
        .file("src/main.cpp")
        .files(&cpp_files)
        .includes(INCLUDE)
        .std("c++14")
        .compile("bitnet");

    cc::Build::new()
        .files(&c_files)
        .includes(INCLUDE)
        .flag_if_supported("-std=c11")
        .compile("ggml_c_files");

    println!("cargo:rerun-if-changed=src/main.cpp");
    println!("cargo:rerun-if-changed=src/bitnet_model.cpp");
    println!("cargo:rerun-if-changed=include/bitnet_model.h");
}

