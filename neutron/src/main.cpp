#include "../include/neutron/core.hpp"
#include <iostream>

#ifndef NDEBUG
    #define DEBUG_LOG(x) std::cerr << x << std::endl
#else
    #define DEBUG_LOG(x)
#endif

auto main() -> int {
    DEBUG_LOG("Banana");
    
    return 0;
}
