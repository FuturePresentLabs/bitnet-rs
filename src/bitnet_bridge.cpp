#include "bitnet_model.h"
#include "cxx.h"

std::unique_ptr<BitNetModel> new_bitnet_model() {
    return std::make_unique<BitNetModel>();
}
