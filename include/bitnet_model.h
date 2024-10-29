#pragma once

#include "rust/cxx.h"
#include "bitnet-rs/src/main.rs.h"

InferenceResponse run_inference(LlamaConfig config);
