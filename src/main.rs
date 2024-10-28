use cxx::CxxVector;

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("bitnet-rs/BitNet/include/ggml-bitnet.h");
        include!("bitnet-rs/BitNet/3rdparty/llama.cpp/ggml/include/ggml.h");
        include!("bitnet-rs/BitNet/3rdparty/llama.cpp/ggml/include/ggml-backend.h");

        // type ggml_init_params;

        // unsafe fn ggml_init(params: *mut ggml_init_params);

        // fn ggml_bitnet_init();
        // fn ggml_bitnet_free();

        include!("bitnet-rs/include/bitnet_model.h");

        // type BitNetModel;

        // fn new_bitnet_model() -> UniquePtr<BitNetModel>;
        fn run() -> i32;

        fn hello_world();

        // fn loadModel(self: &BitNetModel, modelPath: &str) -> bool;
        // fn setInput(self: &BitNetModel, inputData: &CxxVector<f32>) -> bool;
        // fn runInference(self: &BitNetModel) -> bool;
        // fn getOutput(self: &BitNetModel) -> Vec<f32>;
    }

    unsafe extern "C++" {
    }

    // pub struct ggml_init_params {
    //     pub mem_size: usize,
    //     pub mem_buffer: &mut [u8],
    //     pub no_alloc: bool
    // }
}

// use ffi::BitNetModel;

// use ffi::ggml_init_params;

fn main() {
    // let params = ggml_init_params {
    //     mem_size: 1024 * 1024 * 1024,
    //     mem_buffer: *b"",
    //     no_alloc: false
    // };

    unsafe {
        ffi::hello_world();
        ffi::run();
    }


     // // Create a new model instance
    // let model = ffi::new_bitnet_model();

    // // Load the model parameters
    // if !model.loadModel("path/to/model.bin") {
     //    eprintln!("Failed to load model");
     //    return;
    // }

    // // Prepare input data
    // let input_data: Vec<f32> = vec![/* your input data */];
    // let input_vector = CxxVector::from(&input_data);

    // if !model.setInput(&input_vector) {
     //    eprintln!("Failed to set input");
     //    return;
    // }

    // // Run inference
    // if !model.runInference() {
     //    eprintln!("Inference failed");
     //    return;
    // }

    // // Get the output
    // let output = model.getOutput();

    // // Use the output as needed
    // println!("Inference output: {:?}", output);
}
