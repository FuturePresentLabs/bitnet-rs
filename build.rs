use glob::glob;
use std::path::PathBuf;

fn main() {
    println!("cargo:include=/opt/homebrew/opt/openblas/include");

  // Collect all `.cpp` files from `BitNet` and subdirectories
    let mut cpp_files: Vec<PathBuf> = glob("BitNet/**/*.cpp")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter(|path| path.is_file())
        .filter(|path| !path.to_string_lossy().contains("examples"))
        .collect();

    let mut c_files: Vec<PathBuf> = glob("BitNet/**/*.c")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter(|path| path.is_file())
        .collect();

    cpp_files.append(&mut c_files);

    cxx_build::bridge("src/main.rs")
        .file("BitNet/src/ggml-bitnet-lut.cpp")
        .file("BitNet/src/ggml-bitnet-mad.cpp")
        .file("src/bitnet_model.cpp")
        .files(&cpp_files)
        // .file("src/bitnet_bridge.cpp")
        .include("include")
        .include("BitNet/include")
        .include("BitNet/3rdparty/llama.cpp/include")
        .include("BitNet/3rdparty/llama.cpp/common")
        .include("BitNet/3rdparty/llama.cpp/ggml/include")
        .include("BitNet/3rdparty/llama.cpp/ggml/src")
        .include("/opt/homebrew/opt/openblas/include")
        .std("c++14")
        .compile("bitnet");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/ggml-bitnet-mad.cpp");
    println!("cargo:rerun-if-changed=src/ggml-bitnet-lut.cpp");
    println!("cargo:rerun-if-changed=src/ggml-bitnet.h");
}

