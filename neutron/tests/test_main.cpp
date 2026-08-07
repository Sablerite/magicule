#include <cassert>
#include <iostream>
#include <string>

#include "../include/neutron/core.hpp"

auto main() -> int {
  // Call the EXACT SAME function that main.cpp uses
  std::string result = neutron::say_hello();
  std::cout << "say_hello returned: " << result << std::endl;

  // Assert checks a condition and aborts if false
  assert(result == "neutro");

  // If we reach here, the test passed
  std::cout << "✓ Test passed!" << std::endl;
  return 0;
}
