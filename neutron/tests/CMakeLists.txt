add_executable(neutro_tests test_main.cpp)
target_link_libraries(neutro_tests PRIVATE neutro)
add_test(NAME neutro_tests COMMAND neutro_tests)