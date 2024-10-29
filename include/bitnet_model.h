// bitnet_model.h
#pragma once

#include <vector>
#include <string>
#include <memory>
#include "ggml.h"
#include "rust/cxx.h"
#include "bitnet-rs/src/main.rs.h"

int run_cli(rust::Vec<rust::String> rust_strings);
InferenceResponse run_inference(LlamaConfig config);


