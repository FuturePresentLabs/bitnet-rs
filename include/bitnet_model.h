// bitnet_model.h
#pragma once

#include <vector>
#include <string>
#include <memory>
#include "ggml.h"
#include "rust/cxx.h"

void hello_world();
int run();
int run_cli(rust::Vec<rust::String> rust_strings);

int main(int argc, char ** argv);
