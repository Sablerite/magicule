# --- Auto-detect library source files (everything except main.cpp) ---
file(GLOB_RECURSE NEUTRON_LIB_SOURCES CONFIGURE_DEPENDS
    ${CMAKE_CURRENT_SOURCE_DIR}/*.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/*.hpp
)
list(FILTER NEUTRON_LIB_SOURCES EXCLUDE REGEX "main\\.cpp$")

# --- Auto-detect C++20 module interface units ---
file(GLOB_RECURSE NEUTRON_MODULES CONFIGURE_DEPENDS
    ${CMAKE_CURRENT_SOURCE_DIR}/*.cppm
    ${CMAKE_CURRENT_SOURCE_DIR}/*.ixx
)

# ---- Static library (all project code, no main) ----
add_library(neutron STATIC ${NEUTRON_LIB_SOURCES})

if(NEUTRON_MODULES)
    target_sources(neutron
        FILE_SET CXX_MODULES
        FILES ${NEUTRON_MODULES}
    )
endif()

target_include_directories(neutron
    PUBLIC ${PROJECT_SOURCE_DIR}/include
)

# ---- Main executable ----
add_executable(neutron_main main.cpp)
target_link_libraries(neutron_main PRIVATE neutron)
target_include_directories(neutron_main
    PRIVATE ${PROJECT_SOURCE_DIR}/include
)
