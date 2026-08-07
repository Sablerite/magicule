add_executable(neutron_tests test_main.cpp)
target_link_libraries(neutron_tests PRIVATE neutron)
add_test(NAME neutron_tests COMMAND neutron_tests)
